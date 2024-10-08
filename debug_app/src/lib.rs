use wasm_bindgen::prelude::*;
use web_sys::console;


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


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!, you there"));
    
    // accessing global window object
    let window = web_sys::window().expect("Should have a window");

    // getting at the document
    let document = window.document().expect("need a document");

    // create an element
    let elval = document.create_element("p")?;

    // update the element
    elval.set_inner_html("<h1>This is so damn cool...</h1>");

    // append the element to body
    document.body().expect("get a doc body").append_child(&elval)?;

    Ok(())
}
