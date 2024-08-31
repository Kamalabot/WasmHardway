use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue>{
    // this function is available at index.js
    let window = window().expect("No global window avbl");
    let document = window.document().expect("should have a doc on window");
    let body = document.body().expect("body has to be present");

    // meaning there has to be a body tag
    let val = document.create_element("p")?;
    val.set_inner_html("Hey there from rust");

    body.append_child(&val)?;

    Ok(())
   
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32{
    a + b
}
