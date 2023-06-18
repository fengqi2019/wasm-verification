#[no_mangle]
pub extern "C" fn hello() {
    println!("hello wasi");
}


#[no_mangle]
pub extern "C" fn hello_with_arg(arg: *const u8) {
    unsafe {
        println!("hello wasi: {}", *arg);
    }
}