use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// observe there is no start function in this
// and the function is async, as it will await the response
//
#[wasm_bindgen]
pub async fn run(repo: String) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();

    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let win = web_sys::window().unwrap();
    let resp = JsFuture::from(win.fetch_with_request(&request)).await?;

    assert!(resp.is_instance_of::<Response>());
    let resp_val: Response = resp.dyn_into().unwrap();

    let json = JsFuture::from(resp_val.json()?).await?;

    Ok(json)
}
