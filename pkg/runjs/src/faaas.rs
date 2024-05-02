use rquickjs::class::Trace;
use rquickjs::function::Opt;
use rquickjs::{class, methods, module};

use crate::bindings::wasi::http::types::{FieldKey, FieldValue, StatusCode};

#[derive(Trace)]
#[class(rename_all = "camelCase")]
pub struct Request {}

#[methods]
impl Request {
    #[qjs(constructor)]
    pub fn new() -> Self {
        Request {}
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
