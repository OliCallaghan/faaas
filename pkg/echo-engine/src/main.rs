use std::env;

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
use anyhow::Result;
use faaasmq::{MqTaskContext, MqValue};
use tokio::{
    sync::Notify,
    time::{sleep, Duration},
};
use uuid::Uuid;

struct Consumer();

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

        match serde_json::from_slice::<MqTaskContext>(&content) {
            Ok(ctx) => handle_task(channel, ctx).await,
            Err(err) => eprintln!("Error deserialising: {:?}", err),
        }
    }
}

async fn handle_task(channel: &Channel, ctx: MqTaskContext) {
    println!("CTX: {:?}", serde_json::to_string_pretty(&ctx));

    let ms = ctx
        .args
        .get(2)
        .and_then(|ms| {
            if let MqValue::Uint(ms) = ms {
                Some(*ms)
            } else {
                None
            }
        })
        .unwrap_or(100);

    sleep(Duration::from_millis(ms as u64)).await;

    publish_result(channel, &ctx, MqValue::String("\"success\"".to_string()))
        .await
        .expect("failed to publish success")
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
    let mq_routing_key = "mq.invocations.proxy.echo";
    let mq_exchange_name = "amq.direct";

    let mq_queue_name = "proxy-echo";
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
            mq_routing_key,
        ))
        .await
        .unwrap();

    // Register SQL query consumer
    mq_chann
        .basic_consume(
            Consumer(),
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
