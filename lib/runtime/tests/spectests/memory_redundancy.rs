// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/memory_redundancy.wast
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

// Line 5
fn create_module_1() -> Instance {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (type (;2;) (func (result f32)))
      (type (;3;) (func (param i32) (result i32)))
      (func (;0;) (type 0)
        i32.const 0
        i32.const 0
        i32.store
        i32.const 4
        i32.const 0
        i32.store
        i32.const 8
        i32.const 0
        i32.store
        i32.const 12
        i32.const 0
        i32.store)
      (func (;1;) (type 1) (result i32)
        i32.const 8
        i32.const 0
        i32.store
        i32.const 5
        f32.const -0x0p+0 (;=-0;)
        f32.store
        i32.const 8
        i32.load)
      (func (;2;) (type 1) (result i32)
        (local i32 i32)
        i32.const 8
        i32.load
        set_local 0
        i32.const 5
        i32.const -2147483648
        i32.store
        i32.const 8
        i32.load
        set_local 1
        get_local 0
        get_local 1
        i32.add)
      (func (;3;) (type 2) (result f32)
        (local f32)
        i32.const 8
        i32.const 589505315
        i32.store
        i32.const 11
        f32.load
        set_local 0
        i32.const 8
        i32.const 0
        i32.store
        get_local 0)
      (func (;4;) (type 3) (param i32) (result i32)
        i32.const 16)
      (func (;5;) (type 1) (result i32)
        (local i32 i32)
        i32.const 4
        call 4
        set_local 0
        i32.const 4
        call 4
        set_local 1
        get_local 0
        i32.const 42
        i32.store
        get_local 1
        i32.const 43
        i32.store
        get_local 0
        i32.load)
      (memory (;0;) 1 1)
      (export \"zero_everything\" (func 0))
      (export \"test_store_to_load\" (func 1))
      (export \"test_redundant_load\" (func 2))
      (export \"test_dead_store\" (func 3))
      (export \"malloc\" (func 4))
      (export \"malloc_aliasing\" (func 5)))
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

// Line 59
fn c1_l59_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l59_action_invoke");
    let result = instance.call("test_store_to_load", &[]);
    assert_eq!(result, Ok(Some(Value::I32(128 as i32))));
    result.map(|_| ())
}

// Line 60
fn c2_l60_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l60_action_invoke");
    let result = instance.call("zero_everything", &[]);

    result.map(|_| ())
}

// Line 61
fn c3_l61_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l61_action_invoke");
    let result = instance.call("test_redundant_load", &[]);
    assert_eq!(result, Ok(Some(Value::I32(128 as i32))));
    result.map(|_| ())
}

// Line 62
fn c4_l62_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l62_action_invoke");
    let result = instance.call("zero_everything", &[]);

    result.map(|_| ())
}

// Line 63
fn c5_l63_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l63_action_invoke");
    let result = instance.call("test_dead_store", &[]);
    assert_eq!(
        result,
        Ok(Some(Value::F32(
            (0.000000000000000000000000000000000000000000049f32)
        )))
    );
    result.map(|_| ())
}

// Line 64
fn c6_l64_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l64_action_invoke");
    let result = instance.call("zero_everything", &[]);

    result.map(|_| ())
}

// Line 65
fn c7_l65_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l65_action_invoke");
    let result = instance.call("malloc_aliasing", &[]);
    assert_eq!(result, Ok(Some(Value::I32(43 as i32))));
    result.map(|_| ())
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l59_action_invoke(&mut instance);
    c2_l60_action_invoke(&mut instance);
    c3_l61_action_invoke(&mut instance);
    c4_l62_action_invoke(&mut instance);
    c5_l63_action_invoke(&mut instance);
    c6_l64_action_invoke(&mut instance);
    c7_l65_action_invoke(&mut instance);
}
