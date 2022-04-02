
pub struct WasmDbEngine {
    modules: HashMap<String, FunctionModule>,
}

impl WasmDbEngine {
    pub fn new() -> Self {
        WasmDbEngine {

        }
    }
}

struct FunctionModule {
    binary: Vec<u8>,
    function_name: String,
    arguments: Vec<FunctionArg>,
}

enum FunctionArg {
    String,
    Number,
    bool,
}

type FunctionReturn = Vec<Record>;

struct Record {
    key: String,
    values: HashMap<String, ColumnType>,
}

enum ColumnType {
    String,
    Number,
    Bool,
}