use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, console};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    let document = window().unwrap().document().unwrap();
    
    let button = document.create_element("button")?.dyn_into::<HtmlElement>()?;
    button.set_inner_html("Click ME!!!");

    let closure = Closure::wrap(Box::new(move || {
        console::log_1(&"Click me got clicked".into());
    }) as Box<dyn FnMut()>);
    
    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    document.body().unwrap().append_child(&button);

    closure.forget(); // avoid garbage colln of closure
    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
