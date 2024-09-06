#[allow(dead_code)]
// #![allow(warnings)]
use js_sys::Promise;
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

async fn send_chat_completion() -> Result<(), JsValue> {
    // Create request body
    let messages = vec![Message {
        role: "user".into(),
        content: "Where is the sun located?".into(),
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

    // Create headers
    let headers = Headers::new().unwrap();

    headers.append("Content-Type", "application/json").unwrap();

    headers.append("Authorization", "Bearer api-key").unwrap(); // Replace with your OpenAI key

    opts.set_headers(&headers);

    // Create request object
    let request =
        Request::new_with_str_and_init("https://api.openai.com/v1/chat/completions", &opts)?;

    // Send the request using Fetch API
    let window = web_sys::window().unwrap();
    let fetch_response: Promise = window.fetch_with_request(&request);

    // Convert Promise to Future
    let response_value = JsFuture::from(fetch_response).await?;

    // Check if the response is successful
    let response: Response = response_value.dyn_into().unwrap();
    if response.ok() {
        let json = JsFuture::from(response.json()?).await?;
        // Process the response JSON (log it or use it in your app)
        web_sys::console::log_1(&json);
    } else {
        web_sys::console::log_1(&JsValue::from_str("Failed to get a valid response"));
    }

    Ok(())
}
