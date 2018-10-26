use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(str: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(str: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn time(str: &str);
    #[wasm_bindgen(js_namespace = console, js_name = "timeEnd")]
    pub fn time_end(str: &str);
}

#[macro_export]
macro_rules! log {
    ($($tt:tt)*) => {
        $crate::console::log(&format!($($tt)*));
    }
}

#[macro_export]
macro_rules! time {
    ($name:expr => $value:expr) => {{
        $crate::console::time($name);
        let value = $value;
        $crate::console::time_end($name);
        value
    }};
}
