#![allow(unused_imports, dead_code)] // TODO: remove

mod utils;

use self::utils::{
    load_instance_from_file,
    load_instance_from_wat,
    load_module_from_file,
    load_wasm_from_file,
    wat2wasm,
};
use crate::Extern;
use assert_matches::assert_matches;
use wasmi_core::Value;

#[test]
fn test_add() {
    let (mut store, instance) = load_instance_from_wat(include_bytes!("wat/add.wat"));
    let add = instance
        .get_export(&store, "add")
        .and_then(Extern::into_func)
        .unwrap();
    let mut result = [Value::I32(0)];
    add.call(&mut store, &[Value::I32(1), Value::I32(2)], &mut result)
        .unwrap();
    assert_matches!(result, [Value::I32(3)]);
}

#[test]
fn test_swap() {
    let (mut store, instance) = load_instance_from_wat(include_bytes!("wat/swap.wat"));
    let add = instance
        .get_export(&store, "swap")
        .and_then(Extern::into_func)
        .unwrap();
    let mut result = [Value::I32(0), Value::I32(0)];
    add.call(&mut store, &[Value::I32(1), Value::I32(2)], &mut result)
        .unwrap();
    assert_matches!(result, [Value::I32(2), Value::I32(1)]);
}
