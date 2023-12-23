//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use lwjgl_natives_webgl::mem::Java_org_lwjgl_DefaultSysImplementation_getPointerSize;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pointer_size_is_4() {
    // WebAssembly is a 32-bit architecture.
    assert_eq!(Java_org_lwjgl_DefaultSysImplementation_getPointerSize(), 4);
}
