#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_log, *};
use web_sys::console;

use lwjgl::utils::get_java_method;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_wasm_bindgen::Serializer;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Test {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    test_fn: Function,
}

#[wasm_bindgen_test]
pub fn get_java_method_gets_method() {
    const TEST_PASSED_MARKER: &str = "TEST_PASSED";

    let lib = json!({
        "org": {
            "lwjgl": {
                "Test": Test {
                    test_fn: Function::new_no_args(format!("{TEST_PASSED_MARKER}").as_str())
                }
            },
        }
    });
    let lib = lib.serialize(&Serializer::json_compatible()).unwrap();

    console_log!("lib: {:#?}", lib);

    let test_fn = get_java_method("org.lwjgl.Test.testFn", &lib).unwrap();
    console_log!("test_fn: {:#?}", test_fn);
    consoleTable(&lib);

    let test_fn_value = test_fn.call0(&JsValue::NULL);
    console_log!("test_fn_value: {:#?}", test_fn_value);

    todo!("{:#?}", test_fn_value.unwrap());
}

#[wasm_bindgen(module = "/js/util.js")]
extern "C" {
    fn consoleTable(obj: &JsValue);
}
