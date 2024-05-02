#[allow(warnings)]
mod bindings;
mod buffer;
mod console;
mod crypto;

use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use rquickjs::{CatchResultExt, Class, Context, Module, Runtime};

use crate::buffer::js_buffer_mod;
use crate::crypto::js_crypto_mod;

use crate::console::Console;

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, response: ResponseOutparam) {
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
        // require('os') <-- also virtualised
        // require('fs') <-- virtualised

        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();

        ctx.with(|ctx| {
            // Define globals
            let console = Class::instance(ctx.clone(), Console::new()).unwrap();

            // Attach globals to context
            ctx.globals().set("console", console).unwrap();

            // Define minimum stdlib
            Module::declare_def::<js_buffer_mod, _>(ctx.clone(), "buffer").unwrap();
            Module::declare_def::<js_crypto_mod, _>(ctx.clone(), "crypto").unwrap();

            let invoc_mod_res = Module::evaluate(
                ctx.clone(),
                "invocation",
                r#"
                    import { Buffer } from 'buffer'
                    import { randomBytes, pbkdf2Sync } from 'crypto'

                    console.log("hello")

                    function handleFoo() {
                        const x = Buffer.fromArray([1,2,3,4,5]);

                        return x.readUInt32BE();
                    }

                    function handleBar() {
                        return randomBytes(256).readUInt32BE();
                    }

                    function handleBuz() {
                        return pbkdf2Sync('foo', 'bar', 60, 64, 'sha256');
                    }

                    const foo = handleFoo();
                    const bar = handleBar();
                    const buz = handleBuz();

                    console.log(foo);
                    console.log(bar);
                    console.log(buz.toString('hex'));
                "#,
            );

            if let Err(err) = invoc_mod_res {
                let e = ctx.catch();
                println!("Error {:?} {:?}", err, e);
                panic!("Failed to invoke")
            }

            let res_eval = invoc_mod_res.unwrap().finish::<()>().catch(&ctx);

            match res_eval {
                Ok(_) => {
                    let out = body.write().expect("outgoing stream");
                    out.blocking_write_and_flush(b"Success!, wasi:http/proxy world!\n")
                        .expect("writing response");
                    drop(out);
                }
                Err(_err) => {
                    println!("Hello world");
                    let out = body.write().expect("outgoing stream");
                    let msg = _err.to_string();
                    let msg_bytes = msg.as_bytes();

                    out.blocking_write_and_flush(msg_bytes)
                        .expect("writing response");
                    drop(out);
                }
            }
        });

        ResponseOutparam::set(response, Ok(resp));
        OutgoingBody::finish(body, None).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
