const worker = new Worker("./worker.js");

worker.onmessage = (event) => {
  document.getElementById("response").innerText = event.data.result;
}

document.getElementById("submit").addEventListener("click", (event) => {
  event.preventDefault();
  const input1 = document.getElementById("apiKey").value;
  const input2 = document.getElementById("prompt").value;
  console.log(input1, input2);

  worker.postMessage({input1, input2});
} )
// below is used for dynamically imported modules
// const rust = import('./pkg')

// rust
//   .then(m => {
//     return m.fetch_chat_completion("theeh", "this is prompt").then((data) => {
//     console.log(data);
//     })
//   })
// .catch(console.error);
