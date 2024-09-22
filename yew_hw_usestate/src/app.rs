use yew::prelude::*;
// It creates state that can persist between
// function calls, task switches, or UI
// re-renders.

#[function_component]
pub fn App() -> Html {
    // define state to hold input
    let input_val = use_state(|| String::new());
    let display_val = use_state(|| String::new());
    // callback no: 1
    let on_input = {
        let input_value = input_val.clone();
        Callback::from(move |event: InputEvent| {
            let input = event.target_unchecked_into::<web_sys::HtmlInputElement>();
            // following sets the input_val content
            input_value.set(input.value());
        })
    };

    let on_click = {
        // extract the input value
        let input_value = input_val.clone();
        // keep the display value ready
        let display_value = display_val.clone();
        Callback::from(move |_| {
            // build the message from input_val deref
            let message = format!("Hello, {}!", *input_value);
            web_sys::console::log_1(&message.clone().into());
            display_value.set(message)
        })
    };
    // ui elements are rendered directly
    // without any components
    html! {
        <div>
            <input type="text" placeholder="Enter data" oninput={on_input} />
            <button onclick={on_click}>{ "Greet me" }</button>
            <p>{&*display_val}</p>
        </div>
    }
}
