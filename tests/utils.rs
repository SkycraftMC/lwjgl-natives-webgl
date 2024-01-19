#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use lwjgl::utils::get_java_method;

use serde_json::json;

#[wasm_bindgen_test]
pub fn get_java_method_gets_method() {
    const TEST_PASSED_MARKER: &str = "TEST_PASSED";
    // Add the specified structure to the object
    let test_fn = js_sys::Function::new_no_args("() => { return 420 }");

    let test_object = js_sys::Object::new();
    js_sys::Reflect::set(&test_object, &JsValue::from_str("testFn"), &test_fn);

    let lwjgl_object = js_sys::Object::new();
    js_sys::Reflect::set(
        &lwjgl_object,
        &JsValue::from_str("Test"),
        &test_object.into(),
    );

    let org_object = js_sys::Object::new();
    js_sys::Reflect::set(
        &org_object,
        &JsValue::from_str("lwjgl"),
        &lwjgl_object.into(),
    );

    let js_object = js_sys::Object::new();
    js_sys::Reflect::set(&js_object, &JsValue::from_str("org"), &org_object.into());

    let test_fn = get_java_method("org.lwjgl.Test.testFn", &js_object).unwrap();
    let test_fn_value = test_fn.call0(&JsValue::undefined()).unwrap();

    assert_eq!(test_fn_value.as_string().unwrap(), TEST_PASSED_MARKER);
}

#[wasm_bindgen]
pub fn test_fn() -> i32 {
    420
}
