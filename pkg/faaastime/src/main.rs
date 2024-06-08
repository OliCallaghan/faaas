mod handler;
mod registry;
mod state;
mod timed;

use faaasmq::MqTaskContext;
use state::exports::faaas::task::callable::TaskContext;
use state::types::TaskStatus;
use std::env;
use tokio::sync::Notify;

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{
    BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments,
    QueueDeclareArguments,
};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use anyhow::{Error, Result};
use async_trait::async_trait;

use uuid::Uuid;
use wasmtime::{AsContextMut, Store};

use crate::handler::FaaasInvocationHandler;
use crate::registry::FaaasRegistry;
use crate::state::{FaaasTaskView, Faaastime, FaaastimeState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialise FaaAS registry
    let engine = FaaasRegistry::new_engine()?;
    let registry = FaaasRegistry::new(&engine)?;

    // Configure FaaAS invocation handler
    let handler = FaaasInvocationHandler::new(registry, engine);

    // Collect RabbitMQ connection configuration
    let mq_host = env::var("MQ_HOST").expect("MQ_HOST is not set");
    let mq_port = env::var("MQ_PORT")
        .expect("MQ_PORT is not set")
        .parse::<u16>()
        .expect("MQ_PORT is not a number");
    let mq_user = env::var("MQ_USER").expect("MQ_USER is not set");
    let mq_pass = env::var("MQ_PASS").expect("MQ_PASS is not set");

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
    let mq_routing_key = "mq.invocations.tasks";
    let mq_exchange_name = "amq.direct";

    let mq_queue_decl_args =
        QueueDeclareArguments::durable_client_named("task_invocations").finish();

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

    // Register invocation consumer
    mq_chann
        .basic_consume(
            Consumer(handler),
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

struct Consumer(pub FaaasInvocationHandler);

#[async_trait]
impl AsyncConsumer for Consumer {
    async fn consume(
        &mut self,
        mq_chann: &Channel,
        _deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let handler = &self.0;
        let ctx = serde_json::from_slice::<MqTaskContext>(&content).unwrap();

        let id = ctx.id.clone();

        let res = invoke(handler, ctx.into()).await;

        match res {
            Ok(ctx) => match ctx.into_continuation() {
                TaskStatus::Continuation(ctx) => publish_continuation(mq_chann, ctx).await,
                TaskStatus::Done(ctx) => publish_response(mq_chann, ctx).await,
            },
            Err(err) => publish_error(mq_chann, &id, err).await,
        }
    }
}

async fn publish_continuation(mq_chann: &Channel, ctx: TaskContext) {
    println!("Need to use next_task_id");

    let mq_routing_key = "mq.invocations.io.pg";
    let mq_exchange_name = "amq.direct";

    let args = BasicPublishArguments::new(mq_exchange_name, &mq_routing_key);

    let ctx = MqTaskContext::from(ctx);
    let data = serde_json::to_vec(&ctx).unwrap();

    mq_chann
        .basic_publish(BasicProperties::default(), data, args)
        .await
        .unwrap();
}

async fn publish_response(mq_chann: &Channel, ctx: TaskContext) {
    let mq_routing_key = format!("mq.gateway.invocations.{}", ctx.id);
    let mq_exchange_name = "amq.direct";

    let args = BasicPublishArguments::new(mq_exchange_name, &mq_routing_key);
    let data = serde_json::to_vec(&ctx).unwrap();

    mq_chann
        .basic_publish(BasicProperties::default(), data, args)
        .await
        .unwrap();
}

async fn publish_error(mq_chann: &Channel, invocation_id: &str, err: Error) {
    let mq_routing_key = format!("mq.gateway.invocations.{}", invocation_id);
    let mq_exchange_name = "amq.direct";
    let args = BasicPublishArguments::new(mq_exchange_name, &mq_routing_key);

    println!("Failed to execute function");

    mq_chann
        .basic_publish(
            BasicProperties::default(),
            err.to_string().into_bytes(),
            args,
        )
        .await
        .unwrap();
}

async fn invoke(handler: &FaaasInvocationHandler, ctx: TaskContext) -> Result<TaskContext> {
    let FaaasInvocationHandler(registry, engine) = handler;

    let mut store_task = Store::new(&engine, FaaastimeState::new());

    let task_pre = registry.instantiate_pre(&ctx.task_id).await?;

    let (task, _) = Faaastime::instantiate_pre(&mut store_task, &task_pre).await?;

    // Create Task Context
    let ctx = store_task.data_mut().new_task_ctx(ctx)?;

    let task_shape = task
        .faaas_task_identifiable()
        .call_identify(store_task.as_context_mut())
        .await?;

    println!("Identity of workflow: {}", task_shape);

    let task_res = task
        .faaas_task_callable()
        .call_call(store_task.as_context_mut(), ctx)
        .await?;

    println!("Reached");

    match task_res {
        Ok(ctx) => {
            let r = store_task
                .data_mut()
                .table()
                .get(&ctx)
                .expect("context to exist");

            Ok(r.clone())
        }
        Err(err) => Err(anyhow::Error::msg(format!("Error: {:?}", err))),
    }
}
