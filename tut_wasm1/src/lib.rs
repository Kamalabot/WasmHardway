// use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"Loaded Wasm Module".into());
}

#[derive(Serialize, Deserialize)]
struct PostData {
    title: String,
    body: String,
    user_id: u32,
}

// Define your request body
#[derive(Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[wasm_bindgen]
pub async fn post_data(
    url: String,
    title: String,
    body: String,
    user_id: u32,
) -> Result<JsValue, JsValue> {
    let post_data = PostData {
        title,
        body,
        user_id,
    };
    let json = serde_json::to_string(&post_data).unwrap();

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&json));
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();
    let win = web_sys::window().unwrap();
    let resp_value = JsFuture::from(win.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json_resp = JsFuture::from(resp.json()?).await?;
    Ok(json_resp)
}

#[wasm_bindgen]
pub async fn post_openai(url: String, apikey: String, prompt: String) -> Result<JsValue, JsValue> {
    // Create request body
    let messages = vec![Message {
        role: "user".into(),
        content: prompt,
    }];

    let chat_request = ChatCompletionRequest {
        model: "gpt-4".into(),
        messages,
    };

    // Serialize to JSON
    let body = serde_json::to_string(&chat_request).unwrap();

    // Create request options
    let opts = RequestInit::new();

    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&body));
    opts.set_mode(RequestMode::Cors); // adjust if necessary

    let request = Request::new_with_str_and_init(&url, &opts)?;
    let api_key = format!("Bearer {}", apikey);
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();
    request.headers().set("Authorization", &api_key).unwrap();

    let win = web_sys::window().unwrap();
    let resp_value = JsFuture::from(win.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json_resp = JsFuture::from(resp.json()?).await?;
    Ok(json_resp)
}
