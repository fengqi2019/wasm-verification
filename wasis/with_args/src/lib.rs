#[link(wasm_import_module = "wapc")]
extern "C" {
    fn init_args(bd_ptr: *const u8);
    fn return_rs(bd_ptr: *const u8, len: usize);
}


#[no_mangle]
pub extern "C" fn hello_with_arg(len: usize) {
    unsafe {
        let mut args = Vec::<u8>::with_capacity(len);
        init_args(args.as_mut_ptr());
        args.set_len(len);
        args.push(',' as u8);
        args.push('j' as u8);
        args.push('.' as u8);
        let args = String::from_utf8(args).unwrap();
        println!("{}", args);
        let bytes = args.as_bytes();
        return_rs(bytes.as_ptr(), args.as_bytes().len());
    }
}