import * as wasm from "./pkg/rust_syncloader.js"
// the name for the rust_syncloader comes from the cargo.toml name

self.onmessage = ({ data: bytes }) => {
  /**
   * We use the recieved ArrayBuffer to load synchronously initialize as 
   * opposed to async with default export
   * This is using new WebAssembly.Module() and new WebAssembly.Instance()
   */
  wasm.initSync({ module: bytes });
  wasm.greeter("Gentlemennnn....");
  // wasm.add(5, 6);
};
/*
 * fetched bytes is sent back via post message
 */
self.postMessage({ type: "FETCH WASM"});
