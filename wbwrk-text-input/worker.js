self.importScripts("./pkg/wbwrk_text_input.js");

wasm_bindgen("./pkg/wbwrk_text_input_bg.wasm")
  .then(() => {
    console.log("wasm file loaded")
  }).catch(console.error);

self.onmessage = function (event) {
  const {input1, input2} = event.data;
  const result = wasm_bindgen.process_input(input1, input2);
  self.postMessage({result})
}
