import { chars } from "./char-list.js";
let imp = import("./pkg");

let mod;
let counters = []; // this is going to store the counters on the page

imp
  .then(wasm => {
    mod = wasm;
    // after compiling the rust code, the mod will have Counter inside it
    addCounter();
    let btn = document.getElementById("add-counter")
    if (!btn) throw new Error("Unable to find #add-counter");
    btn.addEventListener("click", ev => addCounter());
  })
  .catch(console.error);

function addCounter() {
  let ctr = mod.Counter.new(randomChar(), 0);
  counters.push(ctr);
  update();
}

function update() {
  let container = document.getElementById("container");
  if (!container) throw new Error("No html tag with container ID");
  while (container.hasChildNodes()) {
    if (container.lastChild.id == "add-counter") break;
    container.removeChild(container.lastChild);
  }
  for (var i = 0; i < counters.length; i++) {
    let counter = counters[i];
    // below the cb param is the function 
    container.appendChild(newCounter(counter.key(), counter.count(), ev => {
      counter.increment();
      update();
    }));
  }
}

function randomChar() {
  console.log("randomchar extraction");
  let idx = Math.floor(Math.random() * (chars.length - 1));
  console.log("Curr index is: ", idx);
  let ret = chars.splice(idx, 1)[0];
  console.log("Extracted Char is: ", ret);
  return ret;
}

function newCounter(key, value, cb) {
  let container = document.createElement("div");
  container.setAttribute("class", "counter");
  let title = document.createElement("h1");
  title.appendChild(document.createTextNode("Counter " + key));
  container.appendChild(title);
  container.appendChild(newField('Count', value));
  let plus = document.createElement("button");
  plus.setAttribute("type", "button");
  plus.setAttribute("class", "plus-button");
  plus.appendChild(document.createTextNode("+"));
  plus.addEventListener("click", cb); // this dispatches the function
  container.appendChild(plus);
  return container;
}


function newField(key, value) {
  let ret = document.createElement("div");
  ret.setAttribute("class", "field");
  let name = document.createElement("span");
  name.setAttribute("class", "name");
  name.appendChild(document.createTextNode(key));
  ret.appendChild(name);
  let val = document.createElement("span");
  val.setAttribute("class", "value");
  val.appendChild(document.createTextNode(value));
  ret.appendChild(val);
  return ret;
}
