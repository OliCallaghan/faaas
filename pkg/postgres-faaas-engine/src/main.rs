use std::env;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments,
        QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use anyhow::{Error, Result};
use faaastime::state::{bindings::faaas::task::types::Value, types::TaskContext};
use tokio::sync::Notify;
use tokio_postgres::{Client, Config, NoTls};
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
        let client = &self.0;
        let ctx = serde_json::from_slice::<TaskContext>(&content).unwrap();

        println!("CTX: {:?}", serde_json::to_string_pretty(&ctx));

        match execute_query(client, &ctx).await {
            Ok(mut ctx) => publish_result(channel, &mut ctx)
                .await
                .expect("failed to publish success"),
            Err(err) => publish_err(channel, &ctx, err)
                .await
                .expect("failed to publish error"),
        }
    }
}

async fn execute_query(client: &Client, ctx: &TaskContext) -> Result<TaskContext> {
    let error = anyhow::Error::msg("Missing query");
    let query = ctx.args.get(0).ok_or(error)?;

    if let Value::StrVal(str) = query {
        let mut ctx = ctx.clone();

        ctx.data.insert("sql".into(), Value::StrVal(str.into()));

        Ok(ctx)
    } else {
        Err(anyhow::Error::msg("Query is not a string"))
    }
}

async fn publish_result(mq_chann: &Channel, ctx: &mut TaskContext) -> Result<()> {
    let mq_routing_key = "mq.invocations.tasks";
    let mq_exchange_name = "amq.direct";

    ctx.set_continuation("tasks/dec_pet", vec![]);
    ctx.continuation();

    let ctx = serde_json::to_vec(&ctx).unwrap();

    println!("Sending result: {:?}", ctx);

    let args = BasicPublishArguments::new(mq_exchange_name, mq_routing_key);

    mq_chann
        .basic_publish(BasicProperties::default(), ctx, args)
        .await
        .unwrap();

    Ok(())
}

async fn publish_err(mq_chann: &Channel, task: &TaskContext, err: Error) -> Result<()> {
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

    // Initialize PostgreSQL connection
    let (client, connection) = Config::new()
        .host(&pg_host)
        .port(pg_port)
        .user(&pg_user)
        .password(&pg_pass)
        .connect(NoTls)
        .await?;

    // Spawn a task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Initialize RabbitMQ connection
    let mq_conn = Connection::open(&OpenConnectionArguments::new(
        &mq_host, mq_port, &mq_user, &mq_pass,
    ))
    .await
    .unwrap();

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
    let mq_routing_key = "mq.invocations.io.pg";
    let mq_exchange_name = "amq.direct";

    let mq_queue_decl_args =
        QueueDeclareArguments::durable_client_named("sql_invocations").finish();

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
            mq_routing_key,
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
