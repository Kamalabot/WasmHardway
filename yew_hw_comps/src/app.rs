#![allow(unused_imports)]
#![allow(warnings)]

use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

// following is for prop_or_default
#[derive(Properties, PartialEq)]
pub struct Dprops {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
pub fn DWorld(props: &Dprops) -> Html {
    if props.is_loading.clone() {
        html! { "loading" }
    } else {
        html! { "Hello World" }
    }
}

#[function_component]
pub fn Case1() -> Html {
    html! {<DWorld />}
}

#[function_component]
pub fn Case2() -> Html {
    html! {<DWorld is_loading={true} />}
}
// ending prop or default

#[function_component]
pub fn HelloWorld() -> Html {
    html! { "Hello there..." }
}
// updating the HelloWorld with Props
#[function_component]
fn HelloWorldP(props: &Props) -> Html {
    html! {
    <div>{"Am I loading? - "}<br/>{props.is_loading.clone()}</div>
    }
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
        // none of Dom direct elemets are
        // showing up in the page
        <br/>
        <NewP />
        <br/>
        <HelloWorldP is_loading={false} />
        <br/>
        <DWorld is_loading={true}/>
        <br/>
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
