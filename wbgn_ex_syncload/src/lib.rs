use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str); 
    
}

#[wasm_bindgen]
pub fn greeter(s: &str) {
    log(&format!("Hello syncly {}", s))
}

#[wasm_bindgen]
pub fn add(s: i32, t: i32) -> i32 {
    s + t
}
