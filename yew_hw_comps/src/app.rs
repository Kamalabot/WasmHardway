#![allow(unused_imports)]
#![allow(warnings)]

use yew::prelude::*;
use yew::{function_component, html, Html};

#[function_component]
pub fn HelloWorld() -> Html {
    html! { "Hello there..." }
}

#[function_component]
pub fn NewP() -> Html {
    html! {"Mostly its building up..."}
}

#[function_component]
pub fn App() -> Html {
    html! {
        <>
        <HelloWorld />
        <NewP />
        </>
    }
}
// following is the virtual dom built
// from the above App

// <App>
//   <HelloWorld>
//     <p>"Hello world"</p>
//   </HelloWord>
// </App>
