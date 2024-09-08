Here is a complete tutorial series on using **Wasm with wasm-bindgen in Rust**, from basic to advanced use cases, including examples for POST/GET requests, API calling, and error handling. The project is structured to have the logic in `lib.rs` and the execution in `main.rs`.

### Project Structure

```
my_wasm_project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs
└── static/
    └── index.html
```

### 1. **Cargo.toml**

```toml
[package]
name = "my_wasm_project"
version = "0.1.0"
edition = "2021"

[dependencies]
wasm-bindgen = "0.2.84"
futures = "0.3.19"
reqwest = { version = "0.11", features = ["wasm"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[lib]
crate-type = ["cdylib"]

[dependencies.web-sys]
version = "0.3.54"
features = [
    "console", "Window", "Request", "RequestInit", "Response", "FormData",
    "Headers", "Document", "Element", "EventTarget", "HtmlElement"
]

[dependencies.js-sys]
version = "0.3.54"
```

### 2. **lib.rs**

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Serialize, Deserialize};
use js_sys::Promise;
use wasm_bindgen::JsCast;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"WASM module loaded.".into());
}

// Example 1: Basic interaction with JavaScript via `wasm_bindgen`
#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello, {}!", name).into());
}

// Example 2: Fetch a GET request
#[wasm_bindgen]
pub async fn fetch_data(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

// Example 3: Fetch a POST request with a JSON body
#[derive(Serialize, Deserialize)]
struct PostData {
    title: String,
    body: String,
    user_id: u32,
}

#[wasm_bindgen]
pub async fn post_data(url: String, title: String, body: String, user_id: u32) -> Result<JsValue, JsValue> {
    let post_data = PostData { title, body, user_id };
    let json = serde_json::to_string(&post_data).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(&json)));
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    let json_resp = JsFuture::from(resp.json()?).await?;
    Ok(json_resp)
}

// Example 4: Error handling with GET request
#[wasm_bindgen]
pub async fn fetch_data_with_error_handling(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    if resp.status() != 200 {
        return Err(JsValue::from_str(&format!(
            "Failed to fetch: {}",
            resp.status_text()
        )));
    }

    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

// Example 5: Handling POST errors
#[wasm_bindgen]
pub async fn post_data_with_error_handling(url: String, title: String, body: String, user_id: u32) -> Result<JsValue, JsValue> {
    let post_data = PostData { title, body, user_id };
    let json = serde_json::to_string(&post_data).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(&json)));
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    if resp.status() != 201 {
        return Err(JsValue::from_str(&format!(
            "Failed to post: {}",
            resp.status_text()
        )));
    }

    let json_resp = JsFuture::from(resp.json()?).await?;
    Ok(json_resp)
}

// Example 6: Making an API call (GET) and displaying the response in the browser
#[wasm_bindgen]
pub async fn fetch_api_and_display(url: String) -> Result<(), JsValue> {
    let data = fetch_data(url).await?;
    console::log_1(&data);
    Ok(())
}

// Example 7: Interfacing with DOM - Manipulating HTML
#[wasm_bindgen]
pub fn change_dom_text() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let val = document.create_element("p").unwrap();
    val.set_inner_html("This is a new paragraph added via WASM!");
    body.append_child(&val).unwrap();
}

// Example 8: Handling form data
#[wasm_bindgen]
pub fn handle_form_data(form_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let form = document.get_element_by_id(form_id).unwrap();
    let form_data = web_sys::FormData::new_with_form(&form.dyn_into::<web_sys::HtmlFormElement>().unwrap()).unwrap();

    let value = form_data.get("input_name").as_string().unwrap();
    console::log_1(&format!("Form data received: {}", value).into());
}

// Example 9: Fetch JSON from an API and parse it
#[wasm_bindgen]
pub async fn fetch_and_parse_json(url: String) -> Result<JsValue, JsValue> {
    let data = fetch_data(url).await?;
    let json_data: serde_json::Value = data.into_serde().unwrap();
    console::log_1(&format!("Parsed JSON: {:?}", json_data).into());
    Ok(JsValue::from_serde(&json_data).unwrap())
}

// Example 10: Using Promises in Rust-Wasm for async tasks
#[wasm_bindgen]
pub async fn fetch_async_with_promise(url: String) -> Promise {
    let future = fetch_data(url);
    wasm_bindgen_futures::future_to_promise(async {
        let result = future.await;
        match result {
            Ok(json) => Ok(json),
            Err(err) => Err(err),
        }
    })
}
```

### 3. **main.rs**

In `main.rs`, you would typically interface with the browser directly through JavaScript. Here's how you would set up your HTML to load and run your WASM code.

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WASM Example</title>
    <script type="module">
        import init, { greet, fetch_api_and_display, change_dom_text, handle_form_data } from './pkg/my_wasm_project.js';

        async function run() {
            await init();
            greet("World");
            await fetch_api_and_display("https://jsonplaceholder.typicode.com/posts/1");
            change_dom_text();
            handle_form_data("myForm");
        }
        run();
    </script>
</head>
<body>
    <h1>WASM Example</h1>
    <form id="myForm">
        <input type="text" name="input_name" placeholder="Enter something" />
        <button type="button" onclick="handle_form_data('myForm')">Submit</button>
    </form>
</body>
</html>
```

# To interface the Rust code compiled to WebAssembly (WASM) with a web browser, follow these steps:

### 1. **Setup and Build**

First, ensure you have the necessary tools installed:

- **Rust:** Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **wasm-pack:** Install it using `cargo install wasm-pack`.

Once you have these installed, you'll need to set up your project to build the WASM module and generate the JavaScript bindings.

#### a. **Project Setup**

1. **Create a new Rust project** if you haven't already:
   
   ```bash
   cargo new my_wasm_project
   cd my_wasm_project
   ```

2. **Edit `Cargo.toml`** to include the dependencies for WASM and `wasm-bindgen` as shown in the previous `Cargo.toml` snippet.

3. **Create your `lib.rs`** file in the `src` directory with the code provided earlier.

4. **Add a `static` directory** in the project root and place your `index.html` file in it.

#### b. **Build the WASM**

Use `wasm-pack` to build the project. This will compile your Rust code to WASM and generate the necessary JavaScript bindings.

```bash
wasm-pack build --target web
```

This command creates a `pkg` directory with your WASM and JavaScript files, ready to be used in a web environment.

### 2. **Serve the Files**

To serve your files, you need a local HTTP server. You can use `basic-http-server` or similar tools. Here's how to do it with `basic-http-server`:

1. **Install `basic-http-server`:**
   
   ```bash
   cargo install basic-http-server
   ```

2. **Run the server:**
   
   ```bash
   basic-http-server static
   ```
   
   This command starts a local server and serves files from the `static` directory.

### 3. **Prepare Your HTML**

Ensure your `index.html` file is correctly set up to load the WASM module. Here’s a sample `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WASM Example</title>
    <script type="module">
        import init, { greet, fetch_api_and_display, change_dom_text, handle_form_data } from './pkg/my_wasm_project.js';

        async function run() {
            await init();
            greet("World");
            await fetch_api_and_display("https://jsonplaceholder.typicode.com/posts/1");
            change_dom_text();
            handle_form_data("myForm");
        }
        run();
    </script>
</head>
<body>
    <h1>WASM Example</h1>
    <form id="myForm">
        <input type="text" name="input_name" placeholder="Enter something" />
        <button type="button" onclick="handle_form_data('myForm')">Submit</button>
    </form>
</body>
</html>
```

### 4. **Open in Browser**

Once everything is set up and the server is running, open your browser and navigate to `http://localhost:4000` (or whatever port your server is using). You should see your web page interacting with the WASM module.

### Summary:

1. **Setup** your Rust project and dependencies.
2. **Build** the WASM module using `wasm-pack`.
3. **Serve** the files with a local HTTP server.
4. **Prepare** an HTML file to load and interact with the WASM module.
5. **Open** the project in a web browser.

This setup allows you to interface with the browser using Rust code compiled to WebAssembly, interacting with the DOM, making network requests, and handling various web tasks.
