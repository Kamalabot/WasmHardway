use yew::prelude::*;

#[function_component(Dpp)]
pub fn app() -> Html {
    // this is where the output is happening
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
