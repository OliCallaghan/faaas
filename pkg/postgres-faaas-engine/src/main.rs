use std::{collections::HashMap, env};

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, Channel,
        QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    tls::TlsAdaptor,
    BasicProperties, Deliver,
};
use anyhow::{Error, Result};
use faaasmq::{MqTaskContext, MqValue};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio::sync::Notify;
use tokio_postgres::{Client, Config, SimpleQueryMessage};
use uuid::Uuid;

struct Consumer(pub Client);

#[async_trait::async_trait]
impl AsyncConsumer for Consumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let args = BasicAckArguments::new(deliver.delivery_tag(), false);
        channel.basic_ack(args).await.unwrap();

        let client = &self.0;

        match serde_json::from_slice::<MqTaskContext>(&content) {
            Ok(ctx) => handle_task(client, channel, ctx).await,
            Err(err) => eprintln!("Error deserialising: {:?}", err),
        }
    }
}

async fn handle_task(client: &Client, channel: &Channel, ctx: MqTaskContext) {
    println!("CTX: {:?}", serde_json::to_string_pretty(&ctx));

    match execute_query(client, &ctx).await {
        Ok(res) => publish_result(channel, &ctx, res)
            .await
            .expect("failed to publish success"),
        Err(err) => publish_err(channel, &ctx, err)
            .await
            .expect("failed to publish error"),
    }
}

async fn execute_query(client: &Client, ctx: &MqTaskContext) -> Result<MqValue> {
    let query = ctx.args.get(2).ok_or(anyhow::Error::msg("Missing query"))?;

    if let MqValue::String(query_str) = query {
        let query_res = client.simple_query(query_str).await?;

        let query_data = query_res
            .iter()
            .map(|msg| match msg {
                SimpleQueryMessage::Row(row) => row
                    .columns()
                    .into_iter()
                    .enumerate()
                    .map(|(col_index, col)| {
                        (
                            col.name().to_string(),
                            row.get(col_index).unwrap_or_default().into(),
                        )
                    })
                    .collect::<HashMap<String, MqValue>>(),
                SimpleQueryMessage::CommandComplete(changed) => {
                    HashMap::from([("changed".into(), MqValue::Uint(*changed as u32))])
                }
                _ => HashMap::new(),
            })
            .collect::<Vec<HashMap<_, _>>>();

        let query_data_serial = serde_json::to_string(&query_data)?;
        let query_data_val = MqValue::String(query_data_serial);

        Ok(query_data_val)
    } else {
        Err(anyhow::Error::msg("Query is not a string"))
    }
}

async fn publish_result(mq_chann: &Channel, ctx: &MqTaskContext, res: MqValue) -> Result<()> {
    let task_id = ctx
        .args
        .get(0)
        .ok_or(anyhow::Error::msg("Missing task ID"))
        .and_then(|id| {
            if let MqValue::String(id) = id {
                Ok(id.clone())
            } else {
                Err(anyhow::Error::msg("Task ID is not a string"))
            }
        })?;

    let task_cont_id = ctx
        .args
        .get(1)
        .ok_or(anyhow::Error::msg("Missing task continuation ID"))
        .and_then(|id| {
            if let MqValue::String(id) = id {
                Ok(id.clone())
            } else {
                Err(anyhow::Error::msg("Task continuation ID is not a string"))
            }
        })?;

    let mq_routing_key = format!("mq.invocations.tasks.{}", task_id);
    let mq_exchange_name = "amq.direct";

    let ctx = ctx.continuation(&task_id, &task_cont_id, vec![res]);
    let ctx = serde_json::to_vec(&ctx).unwrap();

    println!("Sending result: {:?}", ctx);

    let args = BasicPublishArguments::new(mq_exchange_name, &mq_routing_key);

    mq_chann
        .basic_publish(BasicProperties::default(), ctx, args)
        .await
        .unwrap();

    Ok(())
}

async fn publish_err(mq_chann: &Channel, task: &MqTaskContext, err: Error) -> Result<()> {
    let mq_routing_key = format!("mq.gateway.invocations.{}", task.id);
    let mq_exchange_name = "amq.direct";

    let args = BasicPublishArguments::new(mq_exchange_name, &mq_routing_key);

    mq_chann
        .basic_publish(
            BasicProperties::default(),
            err.to_string().into_bytes(),
            args,
        )
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Collect RabbitMQ connection configuration
    let mq_host = env::var("MQ_HOST").expect("MQ_HOST is not set");
    let mq_port = env::var("MQ_PORT")
        .expect("MQ_PORT is not set")
        .parse::<u16>()
        .expect("MQ_PORT is not a number");
    let mq_user = env::var("MQ_USER").expect("MQ_USER is not set");
    let mq_pass = env::var("MQ_PASS").expect("MQ_PASS is not set");

    let pg_host = env::var("PG_HOST").expect("PG_HOST is not set");
    let pg_port = env::var("PG_PORT")
        .expect("PG_PORT is not set")
        .parse::<u16>()
        .expect("PG_PORT is not a number");
    let pg_user = env::var("PG_USER").expect("PG_USER is not set");
    let pg_pass = env::var("PG_PASS").expect("PG_PASS is not set");
    let pg_db = env::var("PG_DB").expect("PG_DB is not set");

    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector = MakeTlsConnector::new(connector);

    // Initialize PostgreSQL connection
    let (client, connection) = Config::new()
        .host(&pg_host)
        .port(pg_port)
        .user(&pg_user)
        .password(&pg_pass)
        .dbname(&pg_db)
        .connect(connector)
        .await?;

    // Spawn a task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Initialize RabbitMQ connection
    let mq_conf_tls_adaptor = TlsAdaptor::without_client_auth(None, mq_host.clone())?;
    let mut mq_conf = OpenConnectionArguments::new(&mq_host, mq_port, &mq_user, &mq_pass);
    mq_conf.tls_adaptor(mq_conf_tls_adaptor);

    let mq_conn = Connection::open(&mq_conf).await.unwrap();

    // Connect to RabbitMQ
    mq_conn
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    // Open channel to RabbitMQ
    let mq_chann = mq_conn.open_channel(None).await.unwrap();
    mq_chann
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    // Connect to RabbitMQ invocation queue
    let mq_node_id = Uuid::new_v4().to_string();
    let mq_routing_key = format!("mq.invocations.proxy.sql.pg.{}", pg_db);
    let mq_exchange_name = "amq.direct";

    let mq_queue_name = format!("proxy-sql-pg-{}", pg_db);
    let mq_queue_decl_args = QueueDeclareArguments::new(&mq_queue_name).finish();

    let (resp_queue_name, _, _) = mq_chann
        .queue_declare(mq_queue_decl_args)
        .await
        .unwrap()
        .unwrap();

    // Bind queue to exchange
    mq_chann
        .queue_bind(QueueBindArguments::new(
            &resp_queue_name,
            mq_exchange_name,
            &mq_routing_key,
        ))
        .await
        .unwrap();

    // Register SQL query consumer
    mq_chann
        .basic_consume(
            Consumer(client),
            BasicConsumeArguments::new(&resp_queue_name, &mq_node_id),
        )
        .await
        .unwrap();

    // Consume until ctrl+c received
    println!("ctrl+c to exit");

    let guard = Notify::new();
    guard.notified().await;

    Ok(())
}
