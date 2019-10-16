use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(str: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(str: &str);
}

pub fn time<T>(label: &str, f: impl FnOnce() -> T) -> T {
    web_sys::console::time_with_label(label);
    let value = f();
    web_sys::console::time_end_with_label(label);
    value
}

#[macro_export]
macro_rules! log {
    ($($tt:tt)*) => {
        $crate::console::log(&format!($($tt)*));
    }
}

// From https://github.com/rust-lang/rust/blob/36d4506cc64337bf7dfc1e3120156922e97659c7/src/libstd/macros.rs#L336-L358
#[macro_export]
macro_rules! dbg {
    () => {
        $crate::log!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::log!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { $crate::dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
