use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
pub fn _log(s: &str) {
    log(s)
}


#[macro_export]
macro_rules! real_console_log {
    ($($t:tt)*) => (crate::utilities::_log(&format_args!($($t)*).to_string()))
}


#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ()
}
