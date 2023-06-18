use wasmtime::{Engine, Linker};
use wasmtime_wasi::WasiCtxBuilder;

fn main() {
    let wasm_path: &str = "./target/wasm32-wasi/release/simply_wasi.wasm";

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    let mut store = wasmtime::Store::new(&engine, wasi);
    let module = wasmtime::Module::from_file(store.engine(), wasm_path).unwrap();
    // for import in module.imports() {
    //     println!("{:?}", import);
    // }
    // for export in module.exports() {
    //     println!("{:?}", export);
    // }
    let module = linker.instantiate(&mut store, &module).unwrap();
    for item in module.exports(&mut store) {
        println!("{} {:?}", item.name(), item.into_func());
    }
    // let ins = Instance::new(store, &module, &[]).unwrap();
    // let mut store = wasmtime::Store::new(&engine, wasi);
    // let func = ins.get_func(store, "hello_with_arg").unwrap();
    // println!("{:?}", func);
    let func = module.get_func(&mut store, "hello_with_arg").unwrap();
    println!("{:?}", func.ty(&store));
    // let func = func.typed(&store).unwrap();
    // println!("{:?}", func.typed(&store).unwrap());
}
