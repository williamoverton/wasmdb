use std::error::Error;
use wasmtime::{Config, Engine, Linker, Module, Store};
mod bindings;
use bindings::host;
use wasi_cap_std_sync::{ambient_authority, Dir, WasiCtxBuilder};
use wasi_common::WasiCtx;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

    pub fn run(&mut self, func_name: String, arguments: Vec<String>) -> Result<Vec<(String, Vec<(String, String)>)>, Box<dyn Error>> {
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

        let response_rows: Arc<Mutex<Vec<(String, Vec<(String, String)>)>>> = Arc::new(Mutex::new(vec![]));

        let host_funcs = HostFuncs {
            arguments,
            store: &mut self.store,
            response_rows: response_rows.clone(),
        };

        let ctx = Context {
            runtime_data: Some(host_funcs),
            wasi: default_wasi(),
        };

        let mut store = Store::new(&engine, ctx);

        let instance = linker.instantiate(&mut store, &module)?;

        let start = instance.get_func(&mut store, "_start").unwrap();
        start.call(&mut store, &[], &mut [])?;

        Ok(response_rows.clone().lock().unwrap().clone())
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
    response_rows: Arc<Mutex<Vec<(String, Vec<(String, String)>)>>>,
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
        let val = self.store.get(key.to_string());
        
        if val.len() > 0 {
            return (key.to_string(), val)
        } else {
            return ("".to_string(), vec![])
        }
    }
    fn upsert(&mut self, key: &str, value: Vec<(&str, &str)>) {
        let values: Vec<(String, String)> = value.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self.store.upsert(key.to_string(), values);
    }
    fn return_record(&mut self, value: (&str, Vec<(&str, &str)>)) {
        let key = value.0.to_string();
        let values: Vec<(String, String)> = value.1.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        
        self.response_rows.lock().unwrap().push((key, values));
    }
}

pub struct FunctionModule {
    pub binary: Vec<u8>,
    pub function_name: String
}