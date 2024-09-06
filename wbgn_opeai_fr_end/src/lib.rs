// use js_sys::{JsString, Promise};
use wasm_bindgen_futures:: JsFuture;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window, Request, RequestInit, RequestMode, Response};

// Function to send the request to OpenAI and handle the response
async fn fetch_chat_completion(api_key: String, prompt: String) {
    console::log_1(&format!("Sending request with prompt: {}", prompt).into());

    // Create the request body as a JSON object
    let body = json!({
        "model": "gpt-4",
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 100,
        "temperature": 0.7
    })
    .to_string();

    // Set up the request parameters
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body));

    // Create a new Request object
    let request =
        Request::new_with_str_and_init("https://api.openai.com/v1/chat/completions", &opts)
            .unwrap();
    request
        .headers()
        .set("Authorization", &format!("Bearer {}", api_key))
        .unwrap();
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    // Perform the fetch request
    let window = window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await;

    match response {
        Ok(res_value) => {
            let res: Response = res_value.dyn_into().unwrap();
            let json = JsFuture::from(res.text().unwrap()).await.unwrap();
            let response_text = json.as_string().unwrap();
            update_response_to_frontend(response_text);
        }
        Err(err) => {
            console::log_1(&err);
            update_response_to_frontend("Request failed!".to_string());
        }
    }
}

// Function to update the response on the frontend
fn update_response_to_frontend(response: String) {
    let document = window().unwrap().document().unwrap();
    let response_element = document.get_element_by_id("response").unwrap();
    response_element.set_inner_html(&response);
}

// Entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    let document = window().unwrap().document().unwrap();
    let submit_button = document.get_element_by_id("submit").unwrap();

    let closure = Closure::wrap(Box::new(move || {
        let api_key_element = document.get_element_by_id("apiKey").unwrap();
        let api_key = api_key_element
            .dyn_ref::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();

        let prompt_element = document.get_element_by_id("prompt").unwrap();
        let prompt = prompt_element
            .dyn_ref::<web_sys::HtmlTextAreaElement>()
            .unwrap()
            .value();

        if api_key.is_empty() || prompt.is_empty() {
            update_response_to_frontend("API Key or prompt is missing!".to_string());
            return;
        }

        // Spawn the async task to handle the request
        spawn_local(fetch_chat_completion(api_key, prompt));
    }) as Box<dyn FnMut()>);

    submit_button
        .dyn_ref::<web_sys::HtmlElement>()
        .unwrap()
        .set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
