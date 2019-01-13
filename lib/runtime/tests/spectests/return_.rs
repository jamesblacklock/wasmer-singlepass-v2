// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/return_.wast
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

// Line 3
fn create_module_1() -> Instance {
    let module_str = "(module
      (type (;0;) (func (param i32 i32 i32) (result i32)))
      (type (;1;) (func))
      (type (;2;) (func (result i32)))
      (type (;3;) (func (result i64)))
      (type (;4;) (func (result f32)))
      (type (;5;) (func (result f64)))
      (type (;6;) (func (param i32 i32) (result i32)))
      (func (;0;) (type 1))
      (func (;1;) (type 1)
        return
        i32.ctz
        drop)
      (func (;2;) (type 1)
        return
        i64.ctz
        drop)
      (func (;3;) (type 1)
        return
        f32.neg
        drop)
      (func (;4;) (type 1)
        return
        f64.neg
        drop)
      (func (;5;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 1
          return
          i32.ctz
        end)
      (func (;6;) (type 3) (result i64)
        block (result i64)  ;; label = @1
          i64.const 2
          return
          i64.ctz
        end)
      (func (;7;) (type 4) (result f32)
        block (result f32)  ;; label = @1
          f32.const 0x1.8p+1 (;=3;)
          return
          f32.neg
        end)
      (func (;8;) (type 5) (result f64)
        block (result f64)  ;; label = @1
          f64.const 0x1p+2 (;=4;)
          return
          f64.neg
        end)
      (func (;9;) (type 1)
        return)
      (func (;10;) (type 5) (result f64)
        f64.const 0x1.8p+1 (;=3;)
        return)
      (func (;11;) (type 2) (result i32)
        i32.const 1
        return
        i32.const 2)
      (func (;12;) (type 2) (result i32)
        call 0
        i32.const 2
        return
        i32.const 3)
      (func (;13;) (type 1)
        nop
        call 0
        return)
      (func (;14;) (type 2) (result i32)
        nop
        call 0
        i32.const 3
        return)
      (func (;15;) (type 1)
        block  ;; label = @1
          return
          call 0
        end)
      (func (;16;) (type 1)
        block  ;; label = @1
          call 0
          return
          call 0
        end)
      (func (;17;) (type 1)
        block  ;; label = @1
          nop
          call 0
          return
        end)
      (func (;18;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          nop
          call 0
          i32.const 2
          return
        end)
      (func (;19;) (type 2) (result i32)
        loop (result i32)  ;; label = @1
          i32.const 3
          return
          i32.const 2
        end)
      (func (;20;) (type 2) (result i32)
        loop (result i32)  ;; label = @1
          call 0
          i32.const 4
          return
          i32.const 2
        end)
      (func (;21;) (type 2) (result i32)
        loop (result i32)  ;; label = @1
          nop
          call 0
          i32.const 5
          return
        end)
      (func (;22;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 9
          return
          br 0 (;@1;)
        end)
      (func (;23;) (type 1)
        block  ;; label = @1
          return
          br_if 0 (;@1;)
        end)
      (func (;24;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 8
          return
          i32.const 1
          br_if 0 (;@1;)
          drop
          i32.const 7
        end)
      (func (;25;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 6
          i32.const 9
          return
          br_if 0 (;@1;)
          drop
          i32.const 7
        end)
      (func (;26;) (type 3) (result i64)
        block  ;; label = @1
          i64.const 9
          return
          br_table 0 (;@1;) 0 (;@1;) 0 (;@1;)
        end
        i64.const -1)
      (func (;27;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 10
          return
          i32.const 1
          br_table 0 (;@1;) 0 (;@1;) 0 (;@1;)
          i32.const 7
        end)
      (func (;28;) (type 2) (result i32)
        block (result i32)  ;; label = @1
          i32.const 6
          i32.const 11
          return
          br_table 0 (;@1;) 0 (;@1;)
          i32.const 7
        end)
      (func (;29;) (type 3) (result i64)
        i64.const 7
        return
        return)
      (func (;30;) (type 2) (result i32)
        i32.const 2
        return
        if (result i32)  ;; label = @1
          i32.const 0
        else
          i32.const 1
        end)
      (func (;31;) (type 6) (param i32 i32) (result i32)
        get_local 0
        if (result i32)  ;; label = @1
          i32.const 3
          return
        else
          get_local 1
        end)
      (func (;32;) (type 6) (param i32 i32) (result i32)
        get_local 0
        if (result i32)  ;; label = @1
          get_local 1
        else
          i32.const 4
          return
        end)
      (func (;33;) (type 6) (param i32 i32) (result i32)
        i32.const 5
        return
        get_local 0
        get_local 1
        select)
      (func (;34;) (type 6) (param i32 i32) (result i32)
        get_local 0
        i32.const 6
        return
        get_local 1
        select)
      (func (;35;) (type 2) (result i32)
        i32.const 0
        i32.const 1
        i32.const 7
        return
        select)
      (func (;36;) (type 0) (param i32 i32 i32) (result i32)
        i32.const -1)
      (func (;37;) (type 2) (result i32)
        i32.const 12
        return
        i32.const 2
        i32.const 3
        call 36)
      (func (;38;) (type 2) (result i32)
        i32.const 1
        i32.const 13
        return
        i32.const 3
        call 36)
      (func (;39;) (type 2) (result i32)
        i32.const 1
        i32.const 2
        i32.const 14
        return
        call 36)
      (func (;40;) (type 2) (result i32)
        i32.const 20
        return
        i32.const 1
        i32.const 2
        i32.const 3
        call_indirect (type 0))
      (func (;41;) (type 2) (result i32)
        i32.const 0
        i32.const 21
        return
        i32.const 2
        i32.const 3
        call_indirect (type 0))
      (func (;42;) (type 2) (result i32)
        i32.const 0
        i32.const 1
        i32.const 22
        return
        i32.const 3
        call_indirect (type 0))
      (func (;43;) (type 2) (result i32)
        i32.const 0
        i32.const 1
        i32.const 2
        i32.const 23
        return
        call_indirect (type 0))
      (func (;44;) (type 2) (result i32)
        (local f32)
        i32.const 17
        return
        set_local 0
        i32.const -1)
      (func (;45;) (type 4) (result f32)
        f32.const 0x1.b33334p+0 (;=1.7;)
        return
        f32.load)
      (func (;46;) (type 3) (result i64)
        i64.const 30
        return
        i64.load8_s)
      (func (;47;) (type 2) (result i32)
        i32.const 30
        return
        f64.const 0x1.cp+2 (;=7;)
        f64.store
        i32.const -1)
      (func (;48;) (type 2) (result i32)
        i32.const 2
        i32.const 31
        return
        i64.store
        i32.const -1)
      (func (;49;) (type 2) (result i32)
        i32.const 32
        return
        i32.const 7
        i32.store8
        i32.const -1)
      (func (;50;) (type 2) (result i32)
        i32.const 2
        i32.const 33
        return
        i64.store16
        i32.const -1)
      (func (;51;) (type 4) (result f32)
        f32.const 0x1.b33334p+1 (;=3.4;)
        return
        f32.neg)
      (func (;52;) (type 2) (result i32)
        i32.const 3
        return
        i32.const 10
        i32.add)
      (func (;53;) (type 3) (result i64)
        i64.const 10
        i64.const 45
        return
        i64.sub)
      (func (;54;) (type 2) (result i32)
        i32.const 44
        return
        i32.eqz)
      (func (;55;) (type 2) (result i32)
        i32.const 43
        return
        f64.const 0x1.4p+3 (;=10;)
        f64.le)
      (func (;56;) (type 2) (result i32)
        f32.const 0x1.4p+3 (;=10;)
        i32.const 42
        return
        f32.ne)
      (func (;57;) (type 2) (result i32)
        i32.const 41
        return
        i32.wrap/i64)
      (func (;58;) (type 2) (result i32)
        i32.const 40
        return
        memory.grow)
      (table (;0;) 1 1 anyfunc)
      (memory (;0;) 1)
      (export \"type-i32\" (func 1))
      (export \"type-i64\" (func 2))
      (export \"type-f32\" (func 3))
      (export \"type-f64\" (func 4))
      (export \"type-i32-value\" (func 5))
      (export \"type-i64-value\" (func 6))
      (export \"type-f32-value\" (func 7))
      (export \"type-f64-value\" (func 8))
      (export \"nullary\" (func 9))
      (export \"unary\" (func 10))
      (export \"as-func-first\" (func 11))
      (export \"as-func-mid\" (func 12))
      (export \"as-func-last\" (func 13))
      (export \"as-func-value\" (func 14))
      (export \"as-block-first\" (func 15))
      (export \"as-block-mid\" (func 16))
      (export \"as-block-last\" (func 17))
      (export \"as-block-value\" (func 18))
      (export \"as-loop-first\" (func 19))
      (export \"as-loop-mid\" (func 20))
      (export \"as-loop-last\" (func 21))
      (export \"as-br-value\" (func 22))
      (export \"as-br_if-cond\" (func 23))
      (export \"as-br_if-value\" (func 24))
      (export \"as-br_if-value-cond\" (func 25))
      (export \"as-br_table-index\" (func 26))
      (export \"as-br_table-value\" (func 27))
      (export \"as-br_table-value-index\" (func 28))
      (export \"as-return-value\" (func 29))
      (export \"as-if-cond\" (func 30))
      (export \"as-if-then\" (func 31))
      (export \"as-if-else\" (func 32))
      (export \"as-select-first\" (func 33))
      (export \"as-select-second\" (func 34))
      (export \"as-select-cond\" (func 35))
      (export \"as-call-first\" (func 37))
      (export \"as-call-mid\" (func 38))
      (export \"as-call-last\" (func 39))
      (export \"as-call_indirect-func\" (func 40))
      (export \"as-call_indirect-first\" (func 41))
      (export \"as-call_indirect-mid\" (func 42))
      (export \"as-call_indirect-last\" (func 43))
      (export \"as-set_local-value\" (func 44))
      (export \"as-load-address\" (func 45))
      (export \"as-loadN-address\" (func 46))
      (export \"as-store-address\" (func 47))
      (export \"as-store-value\" (func 48))
      (export \"as-storeN-address\" (func 49))
      (export \"as-storeN-value\" (func 50))
      (export \"as-unary-operand\" (func 51))
      (export \"as-binary-left\" (func 52))
      (export \"as-binary-right\" (func 53))
      (export \"as-test-operand\" (func 54))
      (export \"as-compare-left\" (func 55))
      (export \"as-compare-right\" (func 56))
      (export \"as-convert-operand\" (func 57))
      (export \"as-memory.grow-size\" (func 58))
      (elem (;0;) (i32.const 0) 36))
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

// Line 217
fn c1_l217_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l217_action_invoke");
    let result = instance.call("type-i32", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 218
fn c2_l218_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l218_action_invoke");
    let result = instance.call("type-i64", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 219
fn c3_l219_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l219_action_invoke");
    let result = instance.call("type-f32", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 220
fn c4_l220_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l220_action_invoke");
    let result = instance.call("type-f64", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 222
fn c5_l222_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l222_action_invoke");
    let result = instance.call("type-i32-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 223
fn c6_l223_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l223_action_invoke");
    let result = instance.call("type-i64-value", &[]);
    assert_eq!(result, Ok(Some(Value::I64(2 as i64))));
    result.map(|_| ())
}

// Line 224
fn c7_l224_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l224_action_invoke");
    let result = instance.call("type-f32-value", &[]);
    assert_eq!(result, Ok(Some(Value::F32((3.0f32)))));
    result.map(|_| ())
}

// Line 225
fn c8_l225_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l225_action_invoke");
    let result = instance.call("type-f64-value", &[]);
    assert_eq!(result, Ok(Some(Value::F64((4.0f64)))));
    result.map(|_| ())
}

// Line 227
fn c9_l227_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l227_action_invoke");
    let result = instance.call("nullary", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 228
fn c10_l228_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l228_action_invoke");
    let result = instance.call("unary", &[]);
    assert_eq!(result, Ok(Some(Value::F64((3.0f64)))));
    result.map(|_| ())
}

// Line 230
fn c11_l230_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l230_action_invoke");
    let result = instance.call("as-func-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1 as i32))));
    result.map(|_| ())
}

// Line 231
fn c12_l231_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l231_action_invoke");
    let result = instance.call("as-func-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 232
fn c13_l232_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l232_action_invoke");
    let result = instance.call("as-func-last", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 233
fn c14_l233_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l233_action_invoke");
    let result = instance.call("as-func-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 235
fn c15_l235_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l235_action_invoke");
    let result = instance.call("as-block-first", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 236
fn c16_l236_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l236_action_invoke");
    let result = instance.call("as-block-mid", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 237
fn c17_l237_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l237_action_invoke");
    let result = instance.call("as-block-last", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 238
fn c18_l238_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l238_action_invoke");
    let result = instance.call("as-block-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 240
fn c19_l240_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l240_action_invoke");
    let result = instance.call("as-loop-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 241
fn c20_l241_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l241_action_invoke");
    let result = instance.call("as-loop-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(4 as i32))));
    result.map(|_| ())
}

// Line 242
fn c21_l242_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l242_action_invoke");
    let result = instance.call("as-loop-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(5 as i32))));
    result.map(|_| ())
}

// Line 244
fn c22_l244_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l244_action_invoke");
    let result = instance.call("as-br-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 246
fn c23_l246_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l246_action_invoke");
    let result = instance.call("as-br_if-cond", &[]);
    assert_eq!(result, Ok(None));
    result.map(|_| ())
}

// Line 247
fn c24_l247_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l247_action_invoke");
    let result = instance.call("as-br_if-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(8 as i32))));
    result.map(|_| ())
}

// Line 248
fn c25_l248_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l248_action_invoke");
    let result = instance.call("as-br_if-value-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(9 as i32))));
    result.map(|_| ())
}

// Line 250
fn c26_l250_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l250_action_invoke");
    let result = instance.call("as-br_table-index", &[]);
    assert_eq!(result, Ok(Some(Value::I64(9 as i64))));
    result.map(|_| ())
}

// Line 251
fn c27_l251_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c27_l251_action_invoke");
    let result = instance.call("as-br_table-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(10 as i32))));
    result.map(|_| ())
}

// Line 252
fn c28_l252_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c28_l252_action_invoke");
    let result = instance.call("as-br_table-value-index", &[]);
    assert_eq!(result, Ok(Some(Value::I32(11 as i32))));
    result.map(|_| ())
}

// Line 254
fn c29_l254_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c29_l254_action_invoke");
    let result = instance.call("as-return-value", &[]);
    assert_eq!(result, Ok(Some(Value::I64(7 as i64))));
    result.map(|_| ())
}

// Line 256
fn c30_l256_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c30_l256_action_invoke");
    let result = instance.call("as-if-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2 as i32))));
    result.map(|_| ())
}

// Line 257
fn c31_l257_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c31_l257_action_invoke");
    let result = instance.call("as-if-then", &[Value::I32(1 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 258
fn c32_l258_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c32_l258_action_invoke");
    let result = instance.call("as-if-then", &[Value::I32(0 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 259
fn c33_l259_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c33_l259_action_invoke");
    let result = instance.call("as-if-else", &[Value::I32(0 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(4 as i32))));
    result.map(|_| ())
}

// Line 260
fn c34_l260_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c34_l260_action_invoke");
    let result = instance.call("as-if-else", &[Value::I32(1 as i32), Value::I32(6 as i32)]);
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 262
fn c35_l262_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c35_l262_action_invoke");
    let result = instance.call(
        "as-select-first",
        &[Value::I32(0 as i32), Value::I32(6 as i32)],
    );
    assert_eq!(result, Ok(Some(Value::I32(5 as i32))));
    result.map(|_| ())
}

// Line 263
fn c36_l263_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c36_l263_action_invoke");
    let result = instance.call(
        "as-select-first",
        &[Value::I32(1 as i32), Value::I32(6 as i32)],
    );
    assert_eq!(result, Ok(Some(Value::I32(5 as i32))));
    result.map(|_| ())
}

// Line 264
fn c37_l264_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c37_l264_action_invoke");
    let result = instance.call(
        "as-select-second",
        &[Value::I32(0 as i32), Value::I32(6 as i32)],
    );
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 265
fn c38_l265_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c38_l265_action_invoke");
    let result = instance.call(
        "as-select-second",
        &[Value::I32(1 as i32), Value::I32(6 as i32)],
    );
    assert_eq!(result, Ok(Some(Value::I32(6 as i32))));
    result.map(|_| ())
}

// Line 266
fn c39_l266_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c39_l266_action_invoke");
    let result = instance.call("as-select-cond", &[]);
    assert_eq!(result, Ok(Some(Value::I32(7 as i32))));
    result.map(|_| ())
}

// Line 268
fn c40_l268_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c40_l268_action_invoke");
    let result = instance.call("as-call-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(12 as i32))));
    result.map(|_| ())
}

// Line 269
fn c41_l269_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c41_l269_action_invoke");
    let result = instance.call("as-call-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(13 as i32))));
    result.map(|_| ())
}

// Line 270
fn c42_l270_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c42_l270_action_invoke");
    let result = instance.call("as-call-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(14 as i32))));
    result.map(|_| ())
}

// Line 272
fn c43_l272_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c43_l272_action_invoke");
    let result = instance.call("as-call_indirect-func", &[]);
    assert_eq!(result, Ok(Some(Value::I32(20 as i32))));
    result.map(|_| ())
}

// Line 273
fn c44_l273_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c44_l273_action_invoke");
    let result = instance.call("as-call_indirect-first", &[]);
    assert_eq!(result, Ok(Some(Value::I32(21 as i32))));
    result.map(|_| ())
}

// Line 274
fn c45_l274_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c45_l274_action_invoke");
    let result = instance.call("as-call_indirect-mid", &[]);
    assert_eq!(result, Ok(Some(Value::I32(22 as i32))));
    result.map(|_| ())
}

// Line 275
fn c46_l275_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c46_l275_action_invoke");
    let result = instance.call("as-call_indirect-last", &[]);
    assert_eq!(result, Ok(Some(Value::I32(23 as i32))));
    result.map(|_| ())
}

// Line 277
fn c47_l277_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c47_l277_action_invoke");
    let result = instance.call("as-set_local-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(17 as i32))));
    result.map(|_| ())
}

// Line 279
fn c48_l279_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c48_l279_action_invoke");
    let result = instance.call("as-load-address", &[]);
    assert_eq!(result, Ok(Some(Value::F32((1.7f32)))));
    result.map(|_| ())
}

// Line 280
fn c49_l280_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c49_l280_action_invoke");
    let result = instance.call("as-loadN-address", &[]);
    assert_eq!(result, Ok(Some(Value::I64(30 as i64))));
    result.map(|_| ())
}

// Line 282
fn c50_l282_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c50_l282_action_invoke");
    let result = instance.call("as-store-address", &[]);
    assert_eq!(result, Ok(Some(Value::I32(30 as i32))));
    result.map(|_| ())
}

// Line 283
fn c51_l283_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c51_l283_action_invoke");
    let result = instance.call("as-store-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(31 as i32))));
    result.map(|_| ())
}

// Line 284
fn c52_l284_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c52_l284_action_invoke");
    let result = instance.call("as-storeN-address", &[]);
    assert_eq!(result, Ok(Some(Value::I32(32 as i32))));
    result.map(|_| ())
}

// Line 285
fn c53_l285_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c53_l285_action_invoke");
    let result = instance.call("as-storeN-value", &[]);
    assert_eq!(result, Ok(Some(Value::I32(33 as i32))));
    result.map(|_| ())
}

// Line 287
fn c54_l287_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c54_l287_action_invoke");
    let result = instance.call("as-unary-operand", &[]);
    assert_eq!(result, Ok(Some(Value::F32((3.4f32)))));
    result.map(|_| ())
}

// Line 289
fn c55_l289_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c55_l289_action_invoke");
    let result = instance.call("as-binary-left", &[]);
    assert_eq!(result, Ok(Some(Value::I32(3 as i32))));
    result.map(|_| ())
}

// Line 290
fn c56_l290_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c56_l290_action_invoke");
    let result = instance.call("as-binary-right", &[]);
    assert_eq!(result, Ok(Some(Value::I64(45 as i64))));
    result.map(|_| ())
}

// Line 292
fn c57_l292_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c57_l292_action_invoke");
    let result = instance.call("as-test-operand", &[]);
    assert_eq!(result, Ok(Some(Value::I32(44 as i32))));
    result.map(|_| ())
}

// Line 294
fn c58_l294_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c58_l294_action_invoke");
    let result = instance.call("as-compare-left", &[]);
    assert_eq!(result, Ok(Some(Value::I32(43 as i32))));
    result.map(|_| ())
}

// Line 295
fn c59_l295_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c59_l295_action_invoke");
    let result = instance.call("as-compare-right", &[]);
    assert_eq!(result, Ok(Some(Value::I32(42 as i32))));
    result.map(|_| ())
}

// Line 297
fn c60_l297_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c60_l297_action_invoke");
    let result = instance.call("as-convert-operand", &[]);
    assert_eq!(result, Ok(Some(Value::I32(41 as i32))));
    result.map(|_| ())
}

// Line 299
fn c61_l299_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c61_l299_action_invoke");
    let result = instance.call("as-memory.grow-size", &[]);
    assert_eq!(result, Ok(Some(Value::I32(40 as i32))));
    result.map(|_| ())
}

// Line 302
#[test]
fn c62_l302_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 124, 3, 2, 1, 0, 10, 5, 1, 3, 0, 15, 11,
    ];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 306
#[test]
fn c63_l306_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 124, 3, 2, 1, 0, 10, 6, 1, 4, 0, 1, 15, 11,
    ];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

// Line 310
#[test]
fn c64_l310_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 124, 3, 2, 1, 0, 10, 7, 1, 5, 0, 66, 1, 15,
        11,
    ];
    let module = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(module.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l217_action_invoke(&mut instance);
    c2_l218_action_invoke(&mut instance);
    c3_l219_action_invoke(&mut instance);
    c4_l220_action_invoke(&mut instance);
    c5_l222_action_invoke(&mut instance);
    c6_l223_action_invoke(&mut instance);
    c7_l224_action_invoke(&mut instance);
    c8_l225_action_invoke(&mut instance);
    c9_l227_action_invoke(&mut instance);
    c10_l228_action_invoke(&mut instance);
    c11_l230_action_invoke(&mut instance);
    c12_l231_action_invoke(&mut instance);
    c13_l232_action_invoke(&mut instance);
    c14_l233_action_invoke(&mut instance);
    c15_l235_action_invoke(&mut instance);
    c16_l236_action_invoke(&mut instance);
    c17_l237_action_invoke(&mut instance);
    c18_l238_action_invoke(&mut instance);
    c19_l240_action_invoke(&mut instance);
    c20_l241_action_invoke(&mut instance);
    c21_l242_action_invoke(&mut instance);
    c22_l244_action_invoke(&mut instance);
    c23_l246_action_invoke(&mut instance);
    c24_l247_action_invoke(&mut instance);
    c25_l248_action_invoke(&mut instance);
    c26_l250_action_invoke(&mut instance);
    c27_l251_action_invoke(&mut instance);
    c28_l252_action_invoke(&mut instance);
    c29_l254_action_invoke(&mut instance);
    c30_l256_action_invoke(&mut instance);
    c31_l257_action_invoke(&mut instance);
    c32_l258_action_invoke(&mut instance);
    c33_l259_action_invoke(&mut instance);
    c34_l260_action_invoke(&mut instance);
    c35_l262_action_invoke(&mut instance);
    c36_l263_action_invoke(&mut instance);
    c37_l264_action_invoke(&mut instance);
    c38_l265_action_invoke(&mut instance);
    c39_l266_action_invoke(&mut instance);
    c40_l268_action_invoke(&mut instance);
    c41_l269_action_invoke(&mut instance);
    c42_l270_action_invoke(&mut instance);
    c43_l272_action_invoke(&mut instance);
    c44_l273_action_invoke(&mut instance);
    c45_l274_action_invoke(&mut instance);
    c46_l275_action_invoke(&mut instance);
    c47_l277_action_invoke(&mut instance);
    c48_l279_action_invoke(&mut instance);
    c49_l280_action_invoke(&mut instance);
    c50_l282_action_invoke(&mut instance);
    c51_l283_action_invoke(&mut instance);
    c52_l284_action_invoke(&mut instance);
    c53_l285_action_invoke(&mut instance);
    c54_l287_action_invoke(&mut instance);
    c55_l289_action_invoke(&mut instance);
    c56_l290_action_invoke(&mut instance);
    c57_l292_action_invoke(&mut instance);
    c58_l294_action_invoke(&mut instance);
    c59_l295_action_invoke(&mut instance);
    c60_l297_action_invoke(&mut instance);
    c61_l299_action_invoke(&mut instance);
}
