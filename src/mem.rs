use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn Java_org_lwjgl_DefaultSysImplementation_getPointerSize() -> i32 {
    // WASM is a 32-bit platform
    4
}
