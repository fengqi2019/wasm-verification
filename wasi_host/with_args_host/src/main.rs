use std::sync::{Arc, RwLock};
use wasmtime::{AsContext, Caller, Engine, FuncType, Linker, Memory, StoreContext, Val, ValType};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

fn main() {
    let engine = Engine::default();
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    let mut store = wasmtime::Store::new(&engine, wasi);
    let module = wasmtime::Module::from_file(store.engine(), "./target/wasm32-wasi/release/with_args.wasm").unwrap();
    let context = Arc::new(Context {
        args: "Hello".to_string(),
        rs: Default::default(),
    });
    let callback_type = FuncType::new(vec![ValType::I32, ValType::I32], vec![]);
    let host = context.clone();
    linker.func_new("wapc", "return_rs", callback_type, move |mut caller, params: &[Val], _results: &mut [Val]| {
        let ptr = params[0].i32();
        let len = params[1].i32();
        let memory = get_caller_memory(&mut caller);
        let vec = get_vec_from_memory(caller.as_context(), memory, ptr.unwrap(), len.unwrap());
        let mut rs = host.rs.write().unwrap();
        *rs = Some(vec);
        Ok(())
    }).unwrap();
    let host = context.clone();
    let callback_type = FuncType::new(vec![ValType::I32], vec![]);
    linker.func_new("wapc", "init_args", callback_type, move |mut caller, params, _results| {
        let args_ptr = params[0].i32();
        let memory = get_caller_memory(&mut caller);
        write_bytes_to_memory(caller.as_context(), memory, args_ptr.unwrap(), host.args.as_bytes());
        Ok(())
    }).unwrap();

    let ins = linker.instantiate(&mut store, &module).unwrap();
    let entry = ins.get_func(&mut store, "hello_with_arg").unwrap();

    entry.call(&mut store, &[Val::I32(context.args.as_bytes().len() as i32)], &mut []).unwrap();

    let rs = context.rs.read().unwrap();
    println!("rs: {}", String::from_utf8(rs.clone().unwrap()).unwrap());
}

pub struct  Context {
    args: String,
    rs: RwLock<Option<Vec<u8>>>,
}

// pub(crate) fn return_rs_func(store: impl AsContextMut, host: Arc<Context>) -> Func {
//     let callback_type = FuncType::new(vec![ValType::I32, ValType::I32], vec![]);
//     Func::new(
//         store,
//         callback_type,
//         move |mut caller, params: &[Val], _results: &mut [Val]| {
//             let ptr = params[0].i32();
//             let len = params[1].i32();
//             let memory = get_caller_memory(&mut caller);
//             let vec = get_vec_from_memory(caller.as_context(), memory, ptr.unwrap(), len.unwrap());
//             let mut rs = host.rs.write().unwrap();
//             *rs = Some(vec);
//             Ok(())
//         },
//     )
// }


fn get_caller_memory<T>(caller: &mut Caller<T>) -> Memory {
    let memory = caller.get_export("memory").map(|e| e.into_memory().unwrap());
    memory.unwrap()
}

fn get_vec_from_memory<'a, T: 'a>(store: impl Into<StoreContext<'a, T>>, mem: Memory, ptr: i32, len: i32) -> Vec<u8> {
    let data = mem.data(store);
    data[ptr as usize..(ptr + len) as usize].iter().copied().collect()
}

fn write_bytes_to_memory(store: impl AsContext, memory: Memory, ptr: i32, slice: &[u8]) {
    println!("write_bytes_to_memory {}, {}", ptr, slice.len());
    #[allow(unsafe_code)]
    unsafe {
        let raw = memory.data_ptr(store).offset(ptr as isize);
        raw.copy_from(slice.as_ptr(), slice.len());
    }
}
