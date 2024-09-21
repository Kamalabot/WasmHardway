use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    // use_state ?
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let val = *counter + 1;
            counter.set(val);
        }
    };
    html! {
        <div>
            <button {onclick}>{ "+1 add" }</button>
            <p> { *counter }</p>
        </div>
    }
}
