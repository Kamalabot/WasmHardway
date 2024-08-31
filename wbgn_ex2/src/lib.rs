use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
fn run(){
    // this function is available at index.js
    bare_bones(); 
    using_a_macro();
    using_websys();
}

// correctness of the prgm relies on the correctness of the annotation

#[wasm_bindgen]
extern "C"{
    // use js_namespace to bind console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // log is polymorphic, so we can have multiple types of it
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)] 
    fn log_many(a: &str, b: &str);
}
// following three functions are written and sent to run()

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32{
    a + b
}

fn bare_bones(){
    log("Hello bare bones");
    log_u32(42);
    log_many("rusty", "logging");
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string())) 
}


fn using_a_macro() {
    console_log!("Hello from the macro...")
}

fn using_websys(){
    console::log_1(&"Hello using websys...".into());
    let js: JsValue = 4.into();
    console::log_2(&"Arbitrary values are printed to log".into(), &js);
}
