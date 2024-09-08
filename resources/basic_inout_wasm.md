Here’s a basic example where WebAssembly (Rust) communicates with the front-end by receiving inputs from two text boxes, processing them in Rust, and then returning the result to the front-end.

### 1. **Rust Code (`lib.rs`)**

```rust
use wasm_bindgen::prelude::*;
use web_sys::console;

// Entry point for the WASM module
#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"WASM Module Loaded!".into());
}

// Function that takes two inputs and returns the concatenation
#[wasm_bindgen]
pub fn process_input(input1: &str, input2: &str) -> String {
    format!("Result: {} and {}", input1, input2)
}
```

The function `console::log_1(&"this string".into())` in Rust uses the `web-sys` crate to log messages to the browser's developer console. Here's what happens step by step:

1. `"this string"` is a Rust string literal.
2. `.into()` converts it into a `JsValue` type, which is compatible with JavaScript.
3. `console::log_1` is a method provided by `web-sys` to log one argument to the browser's console.

Effectively, this logs the string `"this string"` in the JavaScript console.

###### Usage of Into() method

The `.into()` method in Rust is part of the `std::convert::Into` trait, which is implemented across many types, not just in `wasm-bindgen` or `web-sys`.

** It allows for a general conversion between types that implement `Into<T>` for some target type `T`.** 

In this context, `into()` is converting the Rust string into a `JsValue`, as `wasm-bindgen` implements `Into<JsValue>` for `&str`. So, `.into()` is ubiquitous in Rust and can be used in many places beyond WebAssembly-related crates.

### 2. **Web Worker (`worker.js`)**

```javascript
self.importScripts('pkg/your_wasm_package.js');

wasm_bindgen('pkg/your_wasm_package_bg.wasm').then(() => {
    console.log("WASM loaded in worker");
}).catch(console.error);

self.onmessage = function (event) {
    const { input1, input2 } = event.data;
    const result = wasm_bindgen.process_input(input1, input2);
    self.postMessage({ result });
};
```

### 3. **JavaScript (`index.js`)**

```javascript
const worker = new Worker('worker.js');

// Listen for results from the worker
worker.onmessage = (event) => {
    document.getElementById('output').innerText = event.data.result;
};

// Handle form submission
document.getElementById('processForm').addEventListener('submit', (event) => {
    event.preventDefault();
    const input1 = document.getElementById('input1').value;
    const input2 = document.getElementById('input2').value;

    worker.postMessage({ input1, input2 });
});
```

### 4. **HTML (`index.html`)**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="style.css">
    <title>WASM Example</title>
</head>
<body>
    <h1>WASM Input Example</h1>
    <form id="processForm">
        <input type="text" id="input1" placeholder="Enter first input" required>
        <input type="text" id="input2" placeholder="Enter second input" required>
        <button type="submit">Submit</button>
    </form>
    <p id="output"></p>
    <script type="module" src="index.js"></script>
</body>
</html>
```

### 5. **CSS (`style.css`)**

```css
body {
    font-family: Arial, sans-serif;
    margin: 20px;
}

input {
    margin: 5px;
}

button {
    padding: 5px 10px;
}

#output {
    margin-top: 20px;
    font-weight: bold;
}
```

### 6. **Cargo.toml**

```toml
[package]
name = "wasm_example"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
features = ["console"]
```

### Steps:

1. Set up a Rust project with `wasm-bindgen` using `wasm-pack` to build the WASM binary.
2. Load the WASM module into the browser using `index.js` and communicate via a web worker.
3. Inputs from the two text boxes are passed to the Rust function, concatenated, and sent back to the front-end for display.i
