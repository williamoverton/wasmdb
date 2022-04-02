use std::error::Error;
use wasmtime::{Config, Engine, Linker, Module, Store};
mod bindings;
use bindings::host;
use wasi_cap_std_sync::{ambient_authority, Dir, WasiCtxBuilder};
use wasi_common::WasiCtx;
use std::collections::HashMap;

mod datastore;
use datastore::DataStore;

pub struct WasmDbEngine {
    modules: HashMap<String, FunctionModule>,
    store: DataStore,
}

impl WasmDbEngine {
    pub fn new() -> Self {
        WasmDbEngine {
            modules: HashMap::new(),
            store: DataStore::new(),
        }
    }

    pub fn add_function_module(&mut self, module: FunctionModule) {
        self.modules.insert(module.function_name.clone(), module);
    }

    fn get_module_by_name(&self, name: &str) -> Option<&FunctionModule> {
        self.modules.get(name)
    }

    pub fn run(&mut self, func_name: String, arguments: Vec<String>) -> Result<(), Box<dyn Error>> {

        let func_module = self.get_module_by_name(&func_name).unwrap();

        let mut config = Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_multi_memory(true);
        config.wasm_module_linking(true);

        // Modules can be compiled through either the text or binary format
        let engine = Engine::new(&config)?;

        
        let module = Module::from_binary(&engine, func_module.binary.as_slice())?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context| &mut cx.wasi)?;

        host::add_to_linker(&mut linker, |ctx| -> &mut HostFuncs {
            ctx.runtime_data.as_mut().unwrap()
        })?;

        let ctx = Context {
            runtime_data: Some(HostFuncs {
                arguments,
                store: &mut self.store,
            }),
            wasi: default_wasi(),
        };

        let mut store = Store::new(&engine, ctx);

        let instance = linker.instantiate(&mut store, &module)?;

        let start = instance.get_func(&mut store, "_start").unwrap();
        start.call(&mut store, &[], &mut [])?;

        Ok(())
    }
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

struct Context<'a> {
    pub wasi: WasiCtx,
    pub runtime_data: Option<HostFuncs<'a>>,
}

#[derive(Debug, Clone)]
pub struct HostFuncs<'a> {
    arguments: Vec<String>,
    store: &'a DataStore,
}

impl <'a> host::Host for HostFuncs<'a> {
    fn print(&mut self, message: &str) {
        println!("> {}", message);
    }
    fn get_args(&mut self) -> Vec<String> {
        self.arguments.clone()
    }
    fn get_all(&mut self) -> Vec<(String, Vec<(String, String)>)> {
        self.store.get_all()
    }
    fn get(&mut self, key: &str) -> (String, Vec<(String, String)>) {
        (key.to_string(), self.store.get(key.to_string()))
    }
    fn upsert(&mut self, key: &str, value: Vec<(&str, &str)>) {
        let values: Vec<(String, String)> = value.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self.store.upsert(key.to_string(), values);
    }
}

pub struct FunctionModule {
    pub binary: Vec<u8>,
    pub function_name: String
}