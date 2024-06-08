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
use amqprs::tls::TlsAdaptor;
use amqprs::{BasicProperties, Deliver};
use anyhow::Result;
use async_trait::async_trait;
use faaasmq::MqTaskContext;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
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

    let mut mq_queue_decl_args = QueueDeclareArguments::exclusive_server_named();
    mq_queue_decl_args.auto_delete(true);

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

    println!("Listening for invocation {}", invoc_id);

    rx
}

async fn publish_invoke(mq_chann: &Channel, id: &str, task_id: &str) -> Result<()> {
    let mq_routing_key = "mq.invocations.tasks";
    let mq_exchange_name = "amq.direct";

    let invoc = MqTaskContext::new(id, task_id);
    let invoc = serde_json::to_vec(&invoc).unwrap();

    // create arguments for basic_publish
    let args = BasicPublishArguments::new(mq_exchange_name, mq_routing_key);

    mq_chann
        .basic_publish(BasicProperties::default(), invoc, args)
        .await
        .unwrap();

    println!("Published {} with task id {}", id, task_id);

    Ok(())
}

async fn invoke(mq_conn: Connection, task_id: &str) -> Result<Response<Full<Bytes>>, Infallible> {
    let mq_chann = mq_conn.open_channel(None).await.unwrap();

    mq_chann
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let id = Uuid::new_v4().to_string();
    let task_id = task_id.to_string();

    println!("Invoking {} with task id {}", id, task_id);

    let resp = consume_resp(&mq_chann, &id).await;

    publish_invoke(&mq_chann, &id, &task_id)
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

    if req.uri().path() == "/health-check" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from_static(b"gateway alive")))
            .unwrap());
    }

    match url_query.get("task_id") {
        // Invoke function by pushing to message queue.
        Some(task_id) => invoke(mq_conn, task_id).await,
        // No function ID specified.
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from_static(b"not found")))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
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
    let mq_conf_tls_adaptor = TlsAdaptor::without_client_auth(None, mq_host.clone())?;
    let mut mq_conf = OpenConnectionArguments::new(&mq_host, mq_port, &mq_user, &mq_pass);
    mq_conf.tls_adaptor(mq_conf_tls_adaptor);

    let mq_conn = Connection::open(&mq_conf).await.unwrap();

    // Connect to RabbitMQ
    mq_conn
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    println!("Listening on 0.0.0.0:3000");

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
