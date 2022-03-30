use wasmtime::{Engine, Instance, Linker, Module};
use wasmtime_wasi::WasiCtxBuilder;
use anyhow::Result;

pub fn init_module_wasmtime(wasm_path: &str) -> Result<Module> {
    let engine = Engine::default();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let store = wasmtime::Store::new(&engine, wasi);
    Ok(wasmtime::Module::from_file(store.engine(), wasm_path)?)
}

pub fn init_instance_wasmtime(wasm_path: &str) -> Result<Instance> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = wasmtime::Store::new(&engine, wasi);
    let module = wasmtime::Module::from_file(store.engine(), wasm_path)?;
    for import in module.imports() {
        println!("{:?}", import);
    }
    for export in module.exports() {
        println!("{:?}", export);
    }
    Ok(linker.instantiate(&mut store, &module)?)
}

#[test]
fn test_init_instance_wasmtime() {
    use comm::current_path;
    current_path();
    init_instance_wasmtime("../sources/simply_wasi.wasm").unwrap();
}