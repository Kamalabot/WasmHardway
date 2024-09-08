#[allow(dead_code)]
// #![allow(warnings)]
// use js_sys::Promise;
use wasm_bindgen::prelude::*;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

// Define your request body
#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[wasm_bindgen]
pub async fn send_chat_completion(prompt: String, apikey: String) -> Result<JsValue, JsValue> {
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

    let api_key = format!("Bearer {}", apikey); // Replace with your OpenAI key

    // Create request object
    let request =
        Request::new_with_str_and_init("https://api.openai.com/v1/chat/completions", &opts)?;

    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();
    request.headers().set("Authorization", &api_key).unwrap();

    // Send the request using Fetch API
    let window = web_sys::window().unwrap();

    // Convert Promise to Future
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Check if the response is successful
    let response: Response = response_value.dyn_into().unwrap();
    let json = JsFuture::from(response.json()?).await?;
    Ok(json)
}
