mod handler;
mod registry;
mod state;
mod timed;

use std::net::SocketAddr;

use anyhow::{bail, Result};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use wasmtime::component::{Component, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store};

use wasmtime_wasi_http::{
    bindings::http::types as http_types, body::HyperOutgoingBody, hyper_response_error,
    WasiHttpView,
};

use crate::handler::ProxyHandler;
use crate::registry::FaaasRegistry;
use crate::state::__with_name0::types::HostTaskContext;
use crate::state::{FaaasTaskView, Faaastime, FaaastimeState};
use crate::timed::TimedExt;

async fn handle(
    ProxyHandler(inner): ProxyHandler,
    req: Request<Incoming>,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    use http_body_util::BodyExt;

    let (sender, receiver) = tokio::sync::oneshot::channel();

    let task = tokio::task::spawn(async move {
        let mut store = Store::new(&inner.engine, FaaastimeState::new());
        let mut store_task = Store::new(&inner.engine, FaaastimeState::new());

        let (task, _inst) = Faaastime::instantiate_pre(&mut store_task, &inner.task_pre).await?;

        let (proxy, _inst) =
            wasmtime_wasi_http::proxy::Proxy::instantiate_pre(&mut store, &inner.component_pre)
                .await?;

        let (mut parts, body) = req.into_parts();

        parts.uri = {
            let uri_parts = parts.uri.into_parts();

            let scheme = uri_parts.scheme.unwrap_or(http::uri::Scheme::HTTP);

            let host = if let Some(val) = parts.headers.get(hyper::header::HOST) {
                std::str::from_utf8(val.as_bytes())
                    .map_err(|_| http_types::ErrorCode::HttpRequestUriInvalid)?
            } else {
                uri_parts
                    .authority
                    .as_ref()
                    .ok_or(http_types::ErrorCode::HttpRequestUriInvalid)?
                    .host()
            };

            let path_with_query = uri_parts
                .path_and_query
                .ok_or(http_types::ErrorCode::HttpRequestUriInvalid)?;

            hyper::Uri::builder()
                .scheme(scheme)
                .authority(host)
                .path_and_query(path_with_query)
                .build()
                .map_err(|_| http_types::ErrorCode::HttpRequestUriInvalid)?
        };

        let req = Request::from_parts(parts, body.map_err(hyper_response_error).boxed());
        let req = store.data_mut().new_incoming_request(req)?;
        let res = store.data_mut().new_response_outparam(sender)?;

        // Create Task Context
        let ctx = store_task.data_mut().new_task_ctx()?;

        let v = task
            .faaas_task_identifiable()
            .call_identify(store_task.as_context_mut())
            .await?;

        println!("Identity of workflow: {}", v);

        let task_res = task
            .faaas_task_callable()
            .call_call(store_task.as_context_mut(), ctx)
            .await?;

        match task_res {
            Ok(ctx) => {
                let val = store_task.data_mut().get(ctx, "a".to_string()).unwrap();
                println!("Value {:?}", val);
            }
            Err(_) => panic!("Something went wrong!"),
        };

        if let Err(e) = proxy
            .wasi_http_incoming_handler()
            .call_handle(store, req, res)
            .await
        {
            println!("Encountered an error executing the javascript! {:?}", e);
            return Err(e);
        }

        Ok(())
    });

    match receiver
        .timed(|_res, elapsed_poll, elapsed_fut| {
            println!(
                "Execution took {:?}, in total, taking {:?}",
                elapsed_poll, elapsed_fut
            )
        })
        .await
    {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            let e = match task.await {
                Ok(r) => r.expect_err("task failed"),
                Err(e) => e.into(),
            };
            bail!("guest never invoked `response-outparam::set` method: {e:?}")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("FaaAS listening on http://{}", addr);

    let engine = FaaasRegistry::new_engine()?;
    let mut registry = FaaasRegistry::new(&engine)?;

    loop {
        let runjs_pre = registry.instantiate_pre("faaas:runjs")?;
        let task_pre = registry.instantiate_pre("task:one")?;

        let handler = ProxyHandler::new(&engine, &runjs_pre, &task_pre);

        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let conn_handler = handler.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| handle(conn_handler.clone(), req)))
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
