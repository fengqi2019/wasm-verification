use std::time::Duration;
use comm::*;
use structopt::StructOpt;
use mem_verification::*;

///
/// 执行命令: cargo run --package mem-verification --release -- -y 0 -t 100
///     -y: wasm文件类型（0：最大文件，1：中等文件，2：最小文件，3，最小文件+clone方式）
///     -t: wasm实例次数，最后结果统计总消耗、单文件实例消耗
/// 大体需要内存：
///     simply_wasi_bigger              2400 kb
///     simply_wasi                      750 kb
///     simply_wasi_strip                530 kb
///     simply_wasi_strip + clone          0 kb
///
#[tokio::main]
async fn main() -> Result<()> {
    log_setting_with_level("info,regalloc=warn,cranelift_codegen=info,wasmer_compiler_cranelift=warn");

    let args: RuntimeArgs = RuntimeArgs::from_args();
    tokio::time::sleep(Duration::from_millis(100)).await;
    match args.wasm_ty {
        0 => check_bigger_wasm_mem(args.times, args.runtime_ty)?,
        1 => check_big_wasm_mem(args.times, args.runtime_ty)?,
        2 => check_small_wasm_mem(args.times, args.runtime_ty)?,
        3 => {
            check_clone_small_wasm_mem(args.times, args.runtime_ty)?
        }
        _ => unreachable!()
    }

    Ok(())
}
