mod engine;
use engine::{WasmDbEngine, FunctionModule};

use std::fs::File;
use std::io::Read;

fn main() {

    let mut engine = WasmDbEngine::new();

    add_modules(&mut engine);

    engine.run("hello".to_string(), vec!["World".to_string(), format!("{}", 32.0)]).unwrap();
}

fn add_modules(engine: &mut WasmDbEngine) {
    let module_bytes = {
        let mut file =
            File::open("./src/builtin_modules/hello/target/wasm32-wasi/release/hello.wasm").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    };

    let module = FunctionModule {
        binary: module_bytes,
        function_name: "hello".to_string()
    };

    engine.add_function_module(module);
}
