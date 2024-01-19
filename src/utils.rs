use js_sys::Function;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/util.js")]
extern "C" {
    fn getMethodFromCJLib(method: &str, lib: &JsValue) -> Option<Function>;
}

pub fn get_java_method(method: &str, lib: &JsValue) -> Option<Function> {
    // call JS
    getMethodFromCJLib(method, lib)
}
