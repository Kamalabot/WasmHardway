use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"Entering Wasm".into())
}

#[wasm_bindgen]
pub fn process_input(in1: &str, in2: &str) -> String {
    println!("entering to process...");
    format!("Result: {} and {}", in1, in2)
}
