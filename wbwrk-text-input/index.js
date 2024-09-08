const worker = new Worker("./worker.js");

worker.onmessage = (event) => {
  document.getElementById("output").innerText = event.data.result;
}

document.getElementById("processForm").addEventListener('submit', (event) => {
  event.preventDefault();

  const input1 = document.getElementById("input1").value;
  const input2 = document.getElementById("input2").value;
  console.log(input2, input1);

  worker.postMessage({input1, input2});
})
