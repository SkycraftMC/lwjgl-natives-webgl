#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use lwjgl::utils::get_java_method;

#[wasm_bindgen_test]
pub fn get_java_method_gets_method() -> Result<(), JsValue> {
    const TEST_PASSED_MARKER: &str = "TEST_PASSED";

    // TODO: use serde_wasm_bindgen::to_value when js_sys::Function serialization is fixed
    let test_fn = Function::new_no_args(&format!("return '{TEST_PASSED_MARKER}'"));
    let test_object = wrap(test_fn, "testFn")?;
    let lwjgl_object = wrap(test_object, "Test")?;
    let org_object = wrap(lwjgl_object, "lwjgl")?;
    let js_object = wrap(org_object, "org")?;

    let test_fn = get_java_method("org.lwjgl.Test.testFn", &js_object).unwrap();
    let test_fn_value = test_fn.call0(&JsValue::NULL);

    assert_eq!(
        test_fn_value.unwrap().as_string().unwrap(),
        TEST_PASSED_MARKER
    );

    Ok(())
}

fn wrap(value: impl Into<JsValue>, key: &str) -> Result<Object, JsValue> {
    let obj = Object::new();
    Reflect::set(&obj, &JsValue::from_str(key), &value.into())?;
    Ok(obj)
}
