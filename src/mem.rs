use std::ffi::c_void;
use std::mem;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn Java_org_lwjgl_DefaultSysImplementation_getPointerSize() -> i32 {
    // This will *always* be 4 on WebAssembly. However, I want to keep the code as close
    // to the original as possible, so I'm checking the size of void* instead
    // *mut c_void is the equivalent of void* in Rust
    mem::size_of::<*mut c_void>() as i32
}
