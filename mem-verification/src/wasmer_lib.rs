use anyhow::Result;
use wasmer::{Cranelift, Module, Store, Universal};

pub fn init_module_wasmer(wasm_path: &str) -> Result<Module> {
    let wasm_bytes = std::fs::read(wasm_path)?;
    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    let module = Module::new(&store, wasm_bytes)?;
    Ok(module)
}
