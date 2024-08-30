use wasm_bindgen::prelude::*;
use web_sys::{console, window, Document, HtmlButtonElement, HtmlInputElement, Window};

#[wasm_bindgen()]
pub fn double_input(input: i32) -> i32 {
    input * 2
}

#[wasm_bindgen]
pub fn concat_strs(s1: &str, s2: &str) -> String {
    format!("{} {}", s1, s2)
}
// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let win: Window = window().unwrap();
    let doc: Document = win.document().unwrap();

    // creating input fields and buttons
    
    let in_field = doc.create_element("input")?.dyn_into::<HtmlInputElement>()?;
    in_field.set_id("inputValue");
    doc.body().unwrap().append_child(&in_field)?;

    let btn = doc.create_element("button")?.dyn_into::<HtmlButtonElement>()?;
    btn.set_inner_html("Process..");
    doc.body().unwrap().append_child(&btn)?;

    let respan = doc.create_element("span")?;
    respan.set_id("result");
    doc.body().unwrap().append_child(&respan)?;

    let clsr = Closure::wrap(Box::new(move || {
        let in_val = doc.get_element_by_id("inputValue") 
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let result = double_input(in_val.parse::<i32>().unwrap_or(0));

        let res_element = doc.get_element_by_id("result").unwrap();
        res_element.set_inner_html(&format!("Result: {}", result));
    }) as Box<dyn FnMut()>);

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    btn.set_onclick(Some(clsr.as_ref().unchecked_ref()));
    clsr.forget();

    Ok(())
}
