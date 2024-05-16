#[allow(warnings)]
mod bindings;
mod buffer;
mod console;
mod crypto;
mod faaas;
mod perf_hooks;

use std::collections::HashMap;

use bindings::wasi::filesystem::types::Descriptor;
use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use rquickjs::function::Args;
use rquickjs::loader::{BuiltinResolver, ModuleLoader};
use rquickjs::{CatchResultExt, Class, Context, Error, Function, Module, Runtime};

use crate::bindings::wasi::filesystem::types::{DescriptorFlags, OpenFlags, PathFlags};
use crate::buffer::js_buffer_mod;
use crate::crypto::js_crypto_mod;
use crate::faaas::{js_faaas_mod, Request, Response};
use crate::perf_hooks::js_perf_hooks_mod;

use crate::console::Console;

struct Component;

impl Descriptor {
    pub fn read_to_string(&self) -> String {
        let mut contents = vec![];
        let mut offset = 0;

        loop {
            let (mut data, more) = self.read(512, offset).expect("read to succeed");
            contents.append(&mut data);
            offset += 512;

            if !more {
                break;
            }
        }

        String::from_utf8(contents).expect("utf8 contents")
    }
}

fn load_entrypoint() -> Result<String, ()> {
    let dirs = bindings::wasi::filesystem::preopens::get_directories();

    let dirs_mapped = dirs
        .into_iter()
        .map(|(d, n)| (n, d))
        .collect::<HashMap<String, Descriptor>>();

    let js_dir = dirs_mapped.get("js").expect("js directory");

    let pflags = PathFlags::empty();
    let oflags = OpenFlags::empty();
    let dflags = DescriptorFlags::READ;

    let entrypoint_file = js_dir
        .open_at(pflags, "entrypoint.js", oflags, dflags)
        .expect("open entrypoint.js");

    let entrypoint_js = entrypoint_file.read_to_string();

    Ok(entrypoint_js)
}

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

        // Define minimum stdlib
        let resolver = BuiltinResolver::default()
            .with_module("buffer")
            .with_module("crypto")
            .with_module("faaas")
            .with_module("perf_hooks");

        let loader = ModuleLoader::default()
            .with_module("buffer", js_buffer_mod)
            .with_module("crypto", js_crypto_mod)
            .with_module("faaas", js_faaas_mod)
            .with_module("perf_hooks", js_perf_hooks_mod);

        rt.set_loader(resolver, loader);

        let req = Request::new();

        ctx.with(|ctx| {
            // Define globals
            let console = Class::instance(ctx.clone(), Console::new()).unwrap();

            // Register globals to context
            ctx.globals().set("console", console).unwrap();

            let (m, p) = Module::declare(
                ctx.clone(),
                "invocation",
                load_entrypoint().expect("handler"),
            )
            .map_err(|_| ctx.catch())
            .unwrap()
            .eval()
            .map_err(|_| ctx.catch())
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
