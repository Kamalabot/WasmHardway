use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlInputElement};

#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"Got in...".into());
    let win = window().unwrap();
    let doc = win.document().unwrap();

    let input_element = doc.get_element_by_id("my_input").unwrap();
    let input_element = input_element.dyn_into::<HtmlInputElement>().unwrap();

    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"oninput callback triggered".into());
    }) as Box<dyn FnMut()>);

    input_element.set_oninput(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
}
