extern crate wasm_bindgen;
extern crate cfg_if;

mod utils;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc`
    // as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

/*
Ensure that we have Rust 1.30 or newer and the wasm32-unknown-unknown
target installed via rustup,
Compile our Rust sources into a WebAssembly .wasm binary via cargo,
Use wasm-bindgen to generate the JavaScript API for using our
Rust-generated WebAssembly.
 */
