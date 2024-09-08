self.importScripts("./pkg/wbgn_opeai_fr_end.js");

wasm_bindgen("./pkg/wbgn_opeai_fr_end_bg.wasm")
  .then(() => {
    console.log("Got the wasm loaded.")
  }).catch(console.error);

self.onmessage = async function (event) {
  const {input1, input2} = event.data;
  try {
    const result = await wasm_bindgen.fetch_chat_completion(input1, input2);
    self.postMessage({ result })
  } catch (err) {
    console.error("Error in worker", err);
    self.postMessage("Error: "+ err);
  }
}
