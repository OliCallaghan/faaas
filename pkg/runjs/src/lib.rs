#[allow(warnings)]
mod bindings;
mod buffer;
mod console;
mod crypto;
mod faaas;

use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use rquickjs::function::Args;
use rquickjs::{CatchResultExt, Class, Context, Error, Function, Module, Runtime};

use crate::buffer::js_buffer_mod;
use crate::crypto::js_crypto_mod;
use crate::faaas::{js_faaas_mod, Request, Response};

use crate::console::Console;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(req_param: IncomingRequest, res_param: ResponseOutparam) {
        // TODO:
        // console.log [DONE]
        // require('buffer') [Need polymorphic constructors still]
        // - buffer.readUInt32BE [DONE]
        // require('net'):
        // - net.Socket()
        // require('tls'):
        // - tls.connect({ socket... })
        // require('crypto')
        // - crypto.randomBytes() [DONE]
        // - crypto.pbkdf2Sync() [DONE]
        // - crypto.createHash() [DONE]
        // - crypto.createHmac() [DONE]
        // require('stream')
        // - Stream.Writable
        // - Stream.Readable
        // - Stream.Duplex
        //   - stream.destroy
        //   - stream.push
        // require('perf_hooks')
        // - performance.now()
        // require('faaas')
        // - Request
        // require('os') <-- also virtualised
        // require('fs') <-- virtualised

        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();

        let req = Request::new();

        ctx.with(|ctx| {
            // Define globals
            let console = Class::instance(ctx.clone(), Console::new()).unwrap();

            // Attach globals to context
            ctx.globals().set("console", console).unwrap();

            // Define minimum stdlib
            Module::declare_def::<js_buffer_mod, _>(ctx.clone(), "buffer").unwrap();
            Module::declare_def::<js_crypto_mod, _>(ctx.clone(), "crypto").unwrap();
            Module::declare_def::<js_faaas_mod, _>(ctx.clone(), "faaas").unwrap();

            let (m, p) = Module::declare(
                ctx.clone(),
                "invocation",
                r#"
                    import { Buffer } from 'buffer'
                    import { randomBytes, pbkdf2Sync } from 'crypto'
                    import { Response } from 'faaas'

                    export function handler(req) {
                        return new Response(200).text("Hello faaas!");
                    }
                "#,
            )
            .unwrap()
            .eval()
            .unwrap();

            p.finish::<()>().catch(&ctx).expect("module to evaluate");

            let handler: Function = m.get("handler").unwrap();
            let mut args = Args::new(ctx.clone(), 1);

            args.push_arg(req).expect("request");

            match handler.call_arg::<Response>(args) {
                Ok(res) => {
                    let hdrs = Fields::from_list(res.fields()).unwrap();
                    let resp = OutgoingResponse::new(hdrs);
                    let body = resp.body().unwrap();

                    resp.set_status_code(res.status_code).unwrap();
                    body.write()
                        .unwrap()
                        .blocking_write_and_flush(res.contents())
                        .unwrap();

                    ResponseOutparam::set(res_param, Ok(resp));
                    OutgoingBody::finish(body, None).unwrap();
                }
                Err(Error::Exception) => panic!("Something went wrong"),
                Err(_) => panic!("Something else went wrong"),
            };
        });
    }
}

bindings::export!(Component with_types_in bindings);
