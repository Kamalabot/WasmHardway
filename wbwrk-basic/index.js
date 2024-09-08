const {startup} = wasm_bindgen;

async function run_wasm() {
  await wasm_bindgen();

  console.log("loading index.js");

  startup();
}

run_wasm();
