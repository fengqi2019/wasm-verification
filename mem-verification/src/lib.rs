pub mod wasmer_lib;
pub mod wasmtime_lib;

use std::fmt::{Debug, Formatter};
use std::path::Path;
use comm::*;
use structopt::StructOpt;
use crate::wasmer_lib::init_module_wasmer;
use crate::wasmtime_lib::{init_instance_wasmtime, init_module_wasmtime};

#[derive(StructOpt)]
pub struct RuntimeArgs {
    #[structopt(short="r", parse(from_str=parse_for_runtime_ty), default_value = "0")]
    pub runtime_ty: u64,
    #[structopt(short="y", parse(from_str=parse_for_ty), default_value = "0")]
    pub wasm_ty: u64,
    #[structopt(short="t", parse(from_str = parse_for_times) , default_value  = "10")]
    pub times: u64,
}

impl Debug for RuntimeArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.wasm_ty, self.times)
    }
}

fn parse_for_runtime_ty(arg: &str) -> u64 {
    match arg {
        "1" => 1,
        "2" => 2,
        _ => 0
    }
}

fn parse_for_ty(arg: &str) -> u64 {
    match arg {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => 0
    }
}
fn parse_for_times(arg: &str) -> u64 {
    match arg {
        "10" => 10,
        "100" => 100,
        _ => 10
    }
}

pub fn check_bigger_wasm_mem(times: u64, runtime_ty: u64) -> Result<()> {
    _check(times, runtime_ty, "./sources/simply_wasi_bigger.wasm")
}
pub fn check_big_wasm_mem(times: u64, runtime_ty: u64) -> Result<()> {
    _check(times, runtime_ty, "sources/simply_wasi.wasm")
}
pub fn check_small_wasm_mem(times: u64, runtime_ty: u64) -> Result<()> {
    _check(times, runtime_ty, "sources/simply_wasi_strip.wasm")
}
pub fn check_clone_small_wasm_mem(times: u64, runtime_ty: u64) -> Result<()> {
    _check(times, runtime_ty, "sources/simply_wasi_strip.wasm")
}

fn _check(times: u64, runtime_ty: u64, wasm_path: &str) -> Result<()> {
    // let path: &Path = Path::new(".");
    // println!("{:?}", path.canonicalize()?);
    let path: &Path = Path::new(wasm_path);
    debug!("{:?}", path.canonicalize()?);
    let init_mem = proc_mem()?;
    let end_mem: u64;
    match runtime_ty {
        0 => {
            let _modules: Vec<wasmtime::Module> = (1..=times).map(|_| init_module_wasmtime(wasm_path).unwrap()).collect();
            end_mem = proc_mem()?;
        },
        1 => {
            let _modules: Vec<wasmer::Module> = (1..=times).map(|_| init_module_wasmer(wasm_path).unwrap()).collect();
            end_mem = proc_mem()?;
        },
        2 => {
            let _modules: Vec<wasmtime::Instance> = (1..=times).map(|_| init_instance_wasmtime(wasm_path).unwrap()).collect();
            end_mem = proc_mem()?;
        }
        _ => {
            unreachable!();
        }
    }
    let total = end_mem - init_mem;
    info!("use big wasm file, total mem: {} kb, each: {} kb", total, total / times);
    Ok(())
}



