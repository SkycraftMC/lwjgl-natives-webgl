use std::ffi::c_void;
use std::mem;

use crate::utils::get_java_method;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn Java_org_lwjgl_DefaultSysImplementation_getJNIVersion(lib: &JsValue) -> i32 {
    let get_required_jni_version =
        get_java_method("org.lwjgl.DefaultSysImplementation.getJNIVersion", lib).unwrap();

    get_required_jni_version
        .call0(&JsValue::undefined())
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

#[wasm_bindgen]
pub fn Java_org_lwjgl_DefaultSysImplementation_getPointerSize() -> i32 {
    // This will *always* be 4 on WebAssembly. However, I want to keep the code as close
    // to the original as possible, so I'm checking the size of void* instead
    // *mut c_void is the equivalent of void* in Rust
    mem::size_of::<*mut c_void>() as i32
}
