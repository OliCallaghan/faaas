use anyhow::{bail, Result};

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::{service_fn, Service};
use hyper::Response;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use wasmtime::component::{Component, InstancePre, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use wasmtime_wasi_http::{
    bindings::http::types as http_types, body::HyperOutgoingBody, hyper_response_error,
    WasiHttpCtx, WasiHttpView,
};

struct FaaastimeState {
    ctx: WasiCtx,
    ctx_http: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for FaaastimeState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for FaaastimeState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.ctx_http
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl FaaastimeState {
    fn new() -> Self {
        let mut wasi = WasiCtxBuilder::new();

        wasi.preopened_dir(
            "js",
            "js",
            wasmtime_wasi::DirPerms::all(),
            wasmtime_wasi::FilePerms::all(),
        )
        .expect("open js");

        Self {
            ctx: wasi.build(),
            ctx_http: WasiHttpCtx {},
            table: ResourceTable::new(),
        }
    }
}

#[derive(Clone)]
struct FaaasHandler {
    engine: Engine,
    component_pre: InstancePre<FaaastimeState>,
}

impl FaaasHandler {
    pub fn new(engine: &Engine, component_pre: &InstancePre<FaaastimeState>) -> Self {
        Self {
            engine: engine.clone(),
            component_pre: component_pre.clone(),
        }
    }
}

type Request = hyper::Request<hyper::body::Incoming>;

#[derive(Clone)]
struct ProxyHandler(Arc<FaaasHandler>);

impl ProxyHandler {
    pub fn new(engine: &Engine, component_pre: &InstancePre<FaaastimeState>) -> Self {
        Self(Arc::new(FaaasHandler::new(engine, component_pre)))
    }
}

async fn handle(
    ProxyHandler(inner): ProxyHandler,
    req: Request,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    use http_body_util::BodyExt;

    let (sender, receiver) = tokio::sync::oneshot::channel();

    let task = tokio::task::spawn(async move {
        let mut store = Store::new(&inner.engine, FaaastimeState::new());

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

        let req = hyper::Request::from_parts(parts, body.map_err(hyper_response_error).boxed());
        let req = store.data_mut().new_incoming_request(req)?;
        let res = store.data_mut().new_response_outparam(sender)?;

        if let Err(e) = proxy
            .wasi_http_incoming_handler()
            .call_handle(store, req, res)
            .await
        {
            return Err(e);
        }

        Ok(())
    });

    match receiver.await {
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

    // Configure engine and handler
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;
    let mut linker = Linker::<FaaastimeState>::new(&engine);

    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    wasmtime_wasi_http::proxy::add_only_http_to_linker(&mut linker)?;

    let runjs_component =
        Component::from_file(&engine, "../runjs/target/wasm32-wasi/release/runjs.wasm")?;
    let runjs_pre = linker.instantiate_pre(&runjs_component)?;

    let handler = ProxyHandler::new(&engine, &runjs_pre);

    loop {
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
