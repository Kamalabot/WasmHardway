// helps to pull the wasm_bindgen module
self.importScripts("./pkg/wbwrk_basic.js")

// intializes the wasm file
wasm_bindgen("./pkg/wbwrk_basic_bg.wasm")
  .then(() => {
    console.log("loaded wasm module");
    // the can be more functions that can be called here
  })
  .catch(console.error);

self.onmessage = function (event) {
  console.log(event.data);
}
