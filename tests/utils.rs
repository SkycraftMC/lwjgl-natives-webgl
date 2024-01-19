#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn get_java_method_gets_method() {
    const TEST_PASSED_MARKER: &str = "TEST_PASSED";
    // oh god.
    let js_object = js_sys::Object::new();

    // Add the specified structure to the object
    js_sys::Reflect::set(&js_object, &JsValue::from_str("org"), &{
        let org_object = js_sys::Object::new();
        js_sys::Reflect::set(&org_object, &JsValue::from_str("lwjgl"), &{
            let lwjgl_object = js_sys::Object::new();
            js_sys::Reflect::set(&lwjgl_object, &JsValue::from_str("Test"), &{
                let test_object = js_sys::Object::new();
                js_sys::Reflect::set(
                    &test_object,
                    &JsValue::from_str("testFn"),
                    &JsValue::from(js_sys::Function::new_no_args(
                        format!("() => {{ return {TEST_PASSED_MARKER} }}").as_str(),
                    )),
                );
                test_object.into()
            });
            lwjgl_object.into()
        });
        org_object.into()
    });

    dbg!(js_object);
}
