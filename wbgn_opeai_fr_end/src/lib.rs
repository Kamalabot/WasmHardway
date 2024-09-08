use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window, Request, RequestInit, RequestMode, Response};

// Function to send the request to OpenAI and handle the response
#[wasm_bindgen]
pub async fn fetch_chat_completion(api_key: String, prompt: String) -> Result<String, JsValue> {
    console::log_1(&format!("Sending request with prompt: {}", prompt).into());

    // Create the request body as a JSON object
    let body = json!({
        "model": "gpt-4o",
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 100,
        "temperature": 0.7
    })
    .to_string();

    // Set up the request parameters
    let opts = RequestInit::new();
    opts.set_method("POST"); // we are doing a post here, while github doing a get
                             // https://platform.openai.com/docs/api-reference/chat/create?lang=curl
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body));
    // let built_opt: JsValue = format!("Here is opts: {:?}", opts).into();
    // console::log_1(&built_opt);
    let request =
        Request::new_with_str_and_init("https://api.openai.com/v1/chat/completions", &opts)?;
    request
        .headers()
        .set("Authorization", &format!("Bearer {}", api_key))?;
    request.headers().set("Content-Type", "application/json")?;

    // let built_req: JsValue =
    //     format!("Here is fully formed request: {:?}", request.to_string()).into();
    // console::log_1(&built_req);

    // Perform the fetch request
    let window = window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    console::log_1(&"reaching here!!!".into());
    let response: Response = response.dyn_into().unwrap();
    console::log_1(&"reaching here next!!!".into());
    let text = JsFuture::from(response.json()?).await?;
    Ok(text.as_string().unwrap_or("no response".to_string()))
}

// Entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"Entering Wasm Realm...".into());
}
