// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/comments.wast
#![allow(
    warnings,
    dead_code
)]
use std::{f32, f64};
use wabt::wat2wasm;

use wasmer_clif_backend::CraneliftCompiler;
use wasmer_runtime::types::Value;
use wasmer_runtime::{module::Module, Instance};

use crate::spectests::_common::{generate_imports, NaNCheck};

// Line 10
fn create_module_1() -> Instance {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(&generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 52

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
}
fn create_module_2() -> Instance {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(&generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_2(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 62

#[test]
fn test_module_2() {
    let mut instance = create_module_2();
    // We group the calls together
    start_module_2(&mut instance);
}
fn create_module_3() -> Instance {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(&generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_3(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 71

#[test]
fn test_module_3() {
    let mut instance = create_module_3();
    // We group the calls together
    start_module_3(&mut instance);
}
fn create_module_4() -> Instance {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new())
        .expect("WASM can't be compiled");
    module
        .instantiate(&generate_imports())
        .expect("WASM can't be instantiated")
}

fn start_module_4(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

#[test]
fn test_module_4() {
    let mut instance = create_module_4();
    // We group the calls together
    start_module_4(&mut instance);
}
