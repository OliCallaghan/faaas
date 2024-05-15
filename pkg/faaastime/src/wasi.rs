use wasi::cli::{stderr, stdin, stdout};
use wasi::clocks::{monotonic_clock, wall_clock};
use wasi::http::types::{IncomingBody, IncomingRequest, ResponseOutparam};
use wasi::http::{self, outgoing_handler};
use wasi::io::poll::HostPollable;
use wasi::io::streams::HostOutputStream;
use wasi::io::{self, poll, streams};
use wasi::random::random;

// Generate bindings for runjs
bindgen!("faaastime");

struct FaaastimeState {}

impl monotonic_clock::Host for FaaastimeState {
    fn now(&mut self) -> wasmtime::Result<monotonic_clock::Instant> {
        todo!()
    }

    fn resolution(&mut self) -> wasmtime::Result<monotonic_clock::Duration> {
        todo!()
    }

    fn subscribe_instant(
        &mut self,
        when: monotonic_clock::Instant,
    ) -> wasmtime::Result<wasmtime::component::Resource<monotonic_clock::Pollable>> {
        todo!()
    }

    fn subscribe_duration(
        &mut self,
        when: monotonic_clock::Duration,
    ) -> wasmtime::Result<wasmtime::component::Resource<monotonic_clock::Pollable>> {
        todo!()
    }
}

impl outgoing_handler::Host for FaaastimeState {
    fn handle(
        &mut self,
        request: wasmtime::component::Resource<outgoing_handler::OutgoingRequest>,
        options: Option<wasmtime::component::Resource<outgoing_handler::RequestOptions>>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<
            wasmtime::component::Resource<outgoing_handler::FutureIncomingResponse>,
            outgoing_handler::ErrorCode,
        >,
    > {
        todo!()
    }
}

impl poll::Host for FaaastimeState {
    fn poll(
        &mut self,
        in_: Vec<wasmtime::component::Resource<poll::Pollable>>,
    ) -> wasmtime::Result<Vec<u32>> {
        todo!()
    }
}

impl HostPollable for FaaastimeState {
    fn block(
        &mut self,
        self_: wasmtime::component::Resource<poll::Pollable>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<poll::Pollable>) -> wasmtime::Result<()> {
        todo!()
    }

    fn ready(
        &mut self,
        self_: wasmtime::component::Resource<poll::Pollable>,
    ) -> wasmtime::Result<bool> {
        todo!()
    }
}

impl stderr::Host for FaaastimeState {
    fn get_stderr(
        &mut self,
    ) -> wasmtime::Result<wasmtime::component::Resource<stderr::OutputStream>> {
        todo!()
    }
}

impl stdin::Host for FaaastimeState {
    fn get_stdin(&mut self) -> wasmtime::Result<wasmtime::component::Resource<stdin::InputStream>> {
        todo!()
    }
}

impl stdout::Host for FaaastimeState {
    fn get_stdout(
        &mut self,
    ) -> wasmtime::Result<wasmtime::component::Resource<stdout::OutputStream>> {
        todo!()
    }
}

impl wall_clock::Host for FaaastimeState {
    fn now(&mut self) -> wasmtime::Result<wall_clock::Datetime> {
        todo!()
    }

    fn resolution(&mut self) -> wasmtime::Result<wall_clock::Datetime> {
        todo!()
    }
}

impl random::Host for FaaastimeState {
    fn get_random_u64(&mut self) -> wasmtime::Result<u64> {
        todo!()
    }

    fn get_random_bytes(&mut self, len: u64) -> wasmtime::Result<Vec<u8>> {
        todo!()
    }
}

impl streams::HostInputStream for FaaastimeState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<streams::InputStream>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<streams::InputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<Vec<u8>, streams::StreamError>> {
        todo!()
    }

    fn blocking_read(
        &mut self,
        self_: wasmtime::component::Resource<streams::InputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<Vec<u8>, streams::StreamError>> {
        todo!()
    }

    fn skip(
        &mut self,
        self_: wasmtime::component::Resource<streams::InputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<u64, streams::StreamError>> {
        todo!()
    }

    fn blocking_skip(
        &mut self,
        self_: wasmtime::component::Resource<streams::InputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<u64, streams::StreamError>> {
        todo!()
    }

    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<streams::InputStream>,
    ) -> wasmtime::Result<wasmtime::component::Resource<streams::Pollable>> {
        todo!()
    }
}

impl HostOutputStream for FaaastimeState {
    fn check_write(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
    ) -> wasmtime::Result<std::prelude::v1::Result<u64, streams::StreamError>> {
        todo!()
    }

    fn write(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
        contents: Vec<u8>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), streams::StreamError>> {
        todo!()
    }

    fn blocking_write_and_flush(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
        contents: Vec<u8>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), streams::StreamError>> {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<streams::OutputStream>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn flush(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), streams::StreamError>> {
        todo!()
    }

    fn splice(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
        src: wasmtime::component::Resource<streams::InputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<u64, streams::StreamError>> {
        todo!()
    }

    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
    ) -> wasmtime::Result<wasmtime::component::Resource<streams::Pollable>> {
        todo!()
    }

    fn write_zeroes(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), streams::StreamError>> {
        todo!()
    }

    fn blocking_flush(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), streams::StreamError>> {
        todo!()
    }

    fn blocking_splice(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
        src: wasmtime::component::Resource<streams::InputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<u64, streams::StreamError>> {
        todo!()
    }

    fn blocking_write_zeroes_and_flush(
        &mut self,
        self_: wasmtime::component::Resource<streams::OutputStream>,
        len: u64,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), streams::StreamError>> {
        todo!()
    }
}

impl streams::Host for FaaastimeState {}

impl http::types::Host for FaaastimeState {
    fn http_error_code(
        &mut self,
        err: wasmtime::component::Resource<http::types::IoError>,
    ) -> wasmtime::Result<Option<http::types::ErrorCode>> {
        todo!()
    }
}

impl http::types::HostResponseOutparam for FaaastimeState {
    fn set(
        &mut self,
        param: wasmtime::component::Resource<ResponseOutparam>,
        response: std::prelude::v1::Result<
            wasmtime::component::Resource<http::types::OutgoingResponse>,
            http::types::ErrorCode,
        >,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<ResponseOutparam>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl http::types::HostRequestOptions for FaaastimeState {
    fn new(
        &mut self,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::RequestOptions>> {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::RequestOptions>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn connect_timeout(
        &mut self,
        self_: wasmtime::component::Resource<http::types::RequestOptions>,
    ) -> wasmtime::Result<Option<http::types::Duration>> {
        todo!()
    }

    fn first_byte_timeout(
        &mut self,
        self_: wasmtime::component::Resource<http::types::RequestOptions>,
    ) -> wasmtime::Result<Option<http::types::Duration>> {
        todo!()
    }

    fn set_connect_timeout(
        &mut self,
        self_: wasmtime::component::Resource<http::types::RequestOptions>,
        duration: Option<http::types::Duration>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }

    fn between_bytes_timeout(
        &mut self,
        self_: wasmtime::component::Resource<http::types::RequestOptions>,
    ) -> wasmtime::Result<Option<http::types::Duration>> {
        todo!()
    }

    fn set_first_byte_timeout(
        &mut self,
        self_: wasmtime::component::Resource<http::types::RequestOptions>,
        duration: Option<http::types::Duration>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }

    fn set_between_bytes_timeout(
        &mut self,
        self_: wasmtime::component::Resource<http::types::RequestOptions>,
        duration: Option<http::types::Duration>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }
}

impl http::types::HostOutgoingResponse for FaaastimeState {
    fn new(
        &mut self,
        headers: wasmtime::component::Resource<http::types::Headers>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::OutgoingResponse>> {
        todo!()
    }

    fn body(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingResponse>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<wasmtime::component::Resource<http::types::OutgoingBody>, ()>,
    > {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::OutgoingResponse>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn headers(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingResponse>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Headers>> {
        todo!()
    }

    fn status_code(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingResponse>,
    ) -> wasmtime::Result<http::types::StatusCode> {
        todo!()
    }

    fn set_status_code(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingResponse>,
        status_code: http::types::StatusCode,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }
}

impl http::types::HostOutgoingRequest for FaaastimeState {
    fn new(
        &mut self,
        headers: wasmtime::component::Resource<http::types::Headers>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::OutgoingRequest>> {
        todo!()
    }

    fn body(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<wasmtime::component::Resource<http::types::OutgoingBody>, ()>,
    > {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn method(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<http::types::Method> {
        todo!()
    }

    fn scheme(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<Option<http::types::Scheme>> {
        todo!()
    }

    fn headers(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Headers>> {
        todo!()
    }

    fn authority(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<Option<String>> {
        todo!()
    }

    fn set_method(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
        method: http::types::Method,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }

    fn set_scheme(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
        scheme: Option<http::types::Scheme>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }

    fn set_authority(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
        authority: Option<String>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }

    fn path_with_query(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
    ) -> wasmtime::Result<Option<String>> {
        todo!()
    }

    fn set_path_with_query(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingRequest>,
        path_with_query: Option<String>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), ()>> {
        todo!()
    }
}

impl http::types::HostOutgoingBody for FaaastimeState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::OutgoingBody>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn write(
        &mut self,
        self_: wasmtime::component::Resource<http::types::OutgoingBody>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<wasmtime::component::Resource<http::types::OutputStream>, ()>,
    > {
        todo!()
    }

    fn finish(
        &mut self,
        this: wasmtime::component::Resource<http::types::OutgoingBody>,
        trailers: Option<wasmtime::component::Resource<http::types::Trailers>>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), http::types::ErrorCode>> {
        todo!()
    }
}

impl http::types::HostIncomingBody for FaaastimeState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::IncomingBody>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn finish(
        &mut self,
        this: wasmtime::component::Resource<http::types::IncomingBody>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::FutureTrailers>> {
        todo!()
    }

    fn stream(
        &mut self,
        self_: wasmtime::component::Resource<http::types::IncomingBody>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<wasmtime::component::Resource<http::types::InputStream>, ()>,
    > {
        todo!()
    }
}

impl http::types::HostIncomingResponse for FaaastimeState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::IncomingResponse>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn status(
        &mut self,
        self_: wasmtime::component::Resource<http::types::IncomingResponse>,
    ) -> wasmtime::Result<http::types::StatusCode> {
        todo!()
    }

    fn consume(
        &mut self,
        self_: wasmtime::component::Resource<http::types::IncomingResponse>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<wasmtime::component::Resource<http::types::IncomingBody>, ()>,
    > {
        todo!()
    }

    fn headers(
        &mut self,
        self_: wasmtime::component::Resource<http::types::IncomingResponse>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Headers>> {
        todo!()
    }
}

impl http::types::HostIncomingRequest for FaaastimeState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn method(
        &mut self,
        self_: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<http::types::Method> {
        todo!()
    }

    fn scheme(
        &mut self,
        self_: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<Option<http::types::Scheme>> {
        todo!()
    }

    fn consume(
        &mut self,
        self_: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<wasmtime::component::Resource<http::types::IncomingBody>, ()>,
    > {
        todo!()
    }

    fn headers(
        &mut self,
        self_: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Headers>> {
        todo!()
    }

    fn authority(
        &mut self,
        self_: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<Option<String>> {
        todo!()
    }

    fn path_with_query(
        &mut self,
        self_: wasmtime::component::Resource<IncomingRequest>,
    ) -> wasmtime::Result<Option<String>> {
        todo!()
    }
}

impl http::types::HostFutureIncomingResponse for FaaastimeState {
    fn get(
        &mut self,
        self_: wasmtime::component::Resource<http::types::FutureIncomingResponse>,
    ) -> wasmtime::Result<
        Option<
            std::prelude::v1::Result<
                std::prelude::v1::Result<
                    wasmtime::component::Resource<http::types::IncomingResponse>,
                    http::types::ErrorCode,
                >,
                (),
            >,
        >,
    > {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::FutureIncomingResponse>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<http::types::FutureIncomingResponse>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Pollable>> {
        todo!()
    }
}

impl http::types::HostFutureTrailers for FaaastimeState {
    fn get(
        &mut self,
        self_: wasmtime::component::Resource<http::types::FutureTrailers>,
    ) -> wasmtime::Result<
        Option<
            std::prelude::v1::Result<
                std::prelude::v1::Result<
                    Option<wasmtime::component::Resource<http::types::Trailers>>,
                    http::types::ErrorCode,
                >,
                (),
            >,
        >,
    > {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::FutureTrailers>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<http::types::FutureTrailers>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Pollable>> {
        todo!()
    }
}

impl http::types::HostFields for FaaastimeState {
    fn get(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
        name: http::types::FieldKey,
    ) -> wasmtime::Result<Vec<http::types::FieldValue>> {
        todo!()
    }

    fn has(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
        name: http::types::FieldKey,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn new(&mut self) -> wasmtime::Result<wasmtime::component::Resource<http::types::Fields>> {
        todo!()
    }

    fn set(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
        name: http::types::FieldKey,
        value: Vec<http::types::FieldValue>,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), http::types::HeaderError>> {
        todo!()
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<http::types::Fields>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn clone(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
    ) -> wasmtime::Result<wasmtime::component::Resource<http::types::Fields>> {
        todo!()
    }

    fn append(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
        name: http::types::FieldKey,
        value: http::types::FieldValue,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), http::types::HeaderError>> {
        todo!()
    }

    fn delete(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
        name: http::types::FieldKey,
    ) -> wasmtime::Result<std::prelude::v1::Result<(), http::types::HeaderError>> {
        todo!()
    }

    fn entries(
        &mut self,
        self_: wasmtime::component::Resource<http::types::Fields>,
    ) -> wasmtime::Result<Vec<(http::types::FieldKey, http::types::FieldValue)>> {
        todo!()
    }

    fn from_list(
        &mut self,
        entries: Vec<(http::types::FieldKey, http::types::FieldValue)>,
    ) -> wasmtime::Result<
        std::prelude::v1::Result<
            wasmtime::component::Resource<http::types::Fields>,
            http::types::HeaderError,
        >,
    > {
        todo!()
    }
}

impl io::error::HostError for FaaastimeState {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<io::error::Error>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn to_debug_string(
        &mut self,
        self_: wasmtime::component::Resource<io::error::Error>,
    ) -> wasmtime::Result<String> {
        todo!()
    }
}

impl io::error::Host for FaaastimeState {}
