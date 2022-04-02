use anyhow::Result;
use wasmtime::{Config, Engine, Linker, Module, Store};
use std::error::Error;
mod bindings;
use bindings::host;
use wasi_cap_std_sync::{ambient_authority, Dir, WasiCtxBuilder};
use wasi_common::WasiCtx;

use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {

    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_multi_memory(true);
    config.wasm_module_linking(true);

    // Modules can be compiled through either the text or binary format
    let engine = Engine::new(&config)?;

    let module_bytes = {
        let mut file = File::open("./src/builtin_modules/hello/target/wasm32-wasi/release/hello.wasm")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        buffer
    };

    let module = Module::from_binary(&engine, module_bytes.as_slice())?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context| &mut cx.wasi)?;

    host::add_to_linker(&mut linker, |ctx| -> &mut HostFuncs {
        ctx.runtime_data.as_mut().unwrap()
    })?;

    let ctx = Context {
        runtime_data: Some(HostFuncs{}),
        wasi: default_wasi(),
    };

    let mut store = Store::new(&engine, ctx);

    let instance = linker.instantiate(&mut store, &module)?;

    let start = instance.get_func(&mut store, "_start").unwrap();
    start.call(&mut store, &[], &mut [])?;

    Ok(())
}

fn default_wasi() -> WasiCtx {
    let mut ctx = WasiCtxBuilder::new().inherit_stdio();
    ctx = ctx
        .preopened_dir(
            Dir::open_ambient_dir("./target", ambient_authority()).unwrap(),
            "cache",
        )
        .unwrap();

    ctx.build()
}

struct Context {
    pub wasi: WasiCtx,
    pub runtime_data: Option<HostFuncs>,
}

#[derive(Debug, Clone, Copy)]
pub struct HostFuncs {
}

impl host::Host for HostFuncs {
    fn print(&mut self, message: &str) {
        println!("> {}", message);
    }
    fn double(&mut self, x: f64) -> f64 {
        x * 2.0
    }
}