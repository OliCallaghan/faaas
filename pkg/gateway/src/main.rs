use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{
    BasicAckArguments, BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments,
    QueueDeclareArguments,
};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use anyhow::Result;
use async_trait::async_trait;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::oneshot::{self, Receiver};
use url::form_urlencoded;
use uuid::Uuid;

struct OnceConsumer {
    tx: Option<oneshot::Sender<Vec<u8>>>,
}

impl OnceConsumer {
    pub fn new(tx: oneshot::Sender<Vec<u8>>) -> Self {
        Self { tx: Some(tx) }
    }
}

#[async_trait]
impl AsyncConsumer for OnceConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        // Send ACK to RabbitMQ
        channel
            .basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false))
            .await
            .unwrap();

        // Consume and return the message from the connection
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(content);
        }
    }
}

async fn consume_resp(mq_chann: &Channel, invoc_id: &str) -> Receiver<Vec<u8>> {
    let (tx, rx) = oneshot::channel::<Vec<u8>>();

    let mq_routing_key = format!("mq.gateway.invocations.{}", invoc_id);
    let mq_exchange_name = "amq.direct";

    let mq_queue_decl_args = QueueDeclareArguments::exclusive_server_named();

    let (resp_queue_name, _, _) = mq_chann
        .queue_declare(mq_queue_decl_args)
        .await
        .unwrap()
        .unwrap();

    mq_chann
        .queue_bind(QueueBindArguments::new(
            &resp_queue_name,
            mq_exchange_name,
            &mq_routing_key,
        ))
        .await
        .unwrap();

    mq_chann
        .basic_consume(
            OnceConsumer::new(tx),
            BasicConsumeArguments::new(&resp_queue_name, invoc_id),
        )
        .await
        .unwrap();

    rx
}

#[derive(Serialize, Deserialize)]
struct Invocation {
    fn_id: String,
    invoc_id: String,
}

impl Invocation {
    pub fn new(fn_id: &str, invoc_id: &str) -> Self {
        Self {
            fn_id: fn_id.to_owned(),
            invoc_id: invoc_id.to_owned(),
        }
    }
}

async fn publish_invoke(mq_chann: &Channel, funct_id: &str, invoc_id: &str) -> Result<()> {
    let mq_routing_key = "mq.invocations";
    let mq_exchange_name = "amq.direct";

    let invoc = Invocation::new(funct_id, invoc_id);
    let invoc = serde_json::to_string(&invoc).unwrap().into_bytes();

    // create arguments for basic_publish
    let args = BasicPublishArguments::new(mq_exchange_name, mq_routing_key);

    mq_chann
        .basic_publish(BasicProperties::default(), invoc, args)
        .await
        .unwrap();

    Ok(())
}

async fn invoke(mq_conn: Connection, fn_id: &str) -> Result<Response<Full<Bytes>>, Infallible> {
    let mq_chann = mq_conn.open_channel(None).await.unwrap();

    mq_chann
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let funct_id = fn_id.to_string();
    let invoc_id = Uuid::new_v4().to_string();

    let resp = consume_resp(&mq_chann, &invoc_id).await;

    publish_invoke(&mq_chann, &funct_id, &invoc_id)
        .await
        .expect("to publish invoke");

    let resp = resp.await;

    mq_chann.close().await.unwrap();

    Ok(Response::new(Full::new(resp.expect("data").into())))
}

async fn handle_invocation_req(
    req: Request<hyper::body::Incoming>,
    mq_conn: Connection,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let url_query = req
        .uri()
        .query()
        .map(|q| q.as_bytes())
        .map(form_urlencoded::parse)
        .map(|p| p.collect::<HashMap<_, _>>())
        .unwrap_or_default();

    match url_query.get("fn_id") {
        // Invoke function by pushing to message queue.
        Some(fn_id) => invoke(mq_conn, fn_id).await,
        // No function ID specified.
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from_static(b"not found")))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

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

    // Accept HTTP requests
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let mq_conn = mq_conn.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| handle_invocation_req(req, mq_conn.clone())),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
