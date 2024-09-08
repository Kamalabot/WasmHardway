// For more comments about what's going on here, check out the `hello_world`
// example


import init, { send_chat_completion } from '../pkg/rust_openai_wbgn.js';

  
document.getElementById("submit").addEventListener("click", (event) => {
  event.preventDefault();
  const apikey = document.getElementById("apiKey").value;
  const prompt = document.getElementById("prompt").value;
  console.log(apikey, prompt);
  await init();
  let r = await send_chat_completion("https://api.openai.com/v1/chat/completions",
            apikey, 
            prompt);
  console.log(r)
})

