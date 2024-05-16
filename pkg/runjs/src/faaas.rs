use rquickjs::class::Trace;
use rquickjs::function::Opt;
use rquickjs::{class, methods, module, Ctx, Exception, Result, Value};

use crate::bindings::wasi::http::types::{FieldKey, FieldValue, IncomingRequest, StatusCode};
use crate::bindings::wasi::io::streams::StreamError;

#[derive(Trace)]
#[class(rename_all = "camelCase")]
pub struct Request {
    #[qjs(skip_trace)]
    req_param: IncomingRequest,
}

#[methods]
impl Request {
    #[qjs(constructor)]
    pub fn new_panic() -> Self {
        panic!("Cannot construct a new request type!")
    }

    #[qjs(skip)]
    pub fn new(req_param: IncomingRequest) -> Self {
        Request { req_param }
    }

    pub fn test<'js>(&self, ctx: Ctx<'js>) -> Result<String> {
        Err(Exception::throw_message(&ctx, "Oh no!!"))
    }

    #[qjs(skip)]
    pub fn body<'js>(&self, ctx: &Ctx<'js>) -> Result<Vec<u8>> {
        let body = self
            .req_param
            .consume()
            .map_err(|_| Exception::throw_message(&ctx, "Couldn't consume body"))?;

        let stream = body
            .stream()
            .map_err(|_| Exception::throw_message(&ctx, "Couldn't get stream"))?;

        let mut buffer = Vec::<u8>::with_capacity(512);

        loop {
            let read_res = stream.blocking_read(512);

            if let Err(StreamError::Closed) = read_res {
                break;
            }

            if let Err(StreamError::LastOperationFailed(err)) = read_res {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Stream read operation failed with {:?}", err),
                ));
            }

            if let Ok(mut read_buf) = read_res {
                buffer.append(&mut read_buf);
            }
        }

        Ok(buffer)
    }

    pub fn text<'js>(&self, ctx: Ctx<'js>) -> Result<String> {
        let bytes = self.body(&ctx)?;
        let str = String::from_utf8(bytes)
            .map_err(|_| Exception::throw_message(&ctx, "Not a safe or valid string"))?;

        Ok(str)
    }

    pub fn json<'js>(&self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        let bytes = self.body(&ctx)?;
        let json = ctx
            .json_parse(bytes)
            .map_err(|_| Exception::throw_message(&ctx, "Couldn't parse JSON"))?;

        Ok(json)
    }
}

#[derive(Clone, Trace)]
#[class(rename_all = "camelCase")]
pub struct Response {
    pub status_code: StatusCode,

    #[qjs(skip_trace)]
    pub content: Vec<u8>,
}

#[methods]
impl Response {
    #[qjs(constructor)]
    pub fn new(status_code: Opt<StatusCode>) -> Self {
        Response {
            status_code: status_code.unwrap_or(200),
            content: vec![],
        }
    }

    pub fn text(self, content: String) -> Self {
        Response {
            content: content.as_bytes().to_owned(),
            ..self
        }
    }

    #[qjs(skip)]
    pub fn fields(&self) -> &[(FieldKey, FieldValue)] {
        &[]
    }

    #[qjs(skip)]
    pub fn contents(&self) -> &[u8] {
        &self.content
    }
}

#[module]
pub mod faaas_mod {
    pub use super::Request;
    pub use super::Response;
}
