#![allow(unused_imports)]
// understood how the callback are implementend
// in the parent, and then assigned to child
// then child emits the data from within to the
// console/ parent
// There is no States used here.
use yew::prelude::*;

// Define properties for the Child component, including a callback
#[derive(Properties, PartialEq)]
pub struct ChildProps {
    pub on_data_send: Callback<String>,
}

// Child component that triggers the callback
#[function_component]
pub fn Child(props: &ChildProps) -> Html {
    // This button triggers the callback with some
    // data when clicked
    let on_button_click = {
        let on_data_send = props.on_data_send.clone();
        move |_| {
            // Emit the data to the parent when the button is clicked
            on_data_send.emit("Hello from Child".to_string());
        }
    };
    // lets assume I want the cb to emit when the
    // component is rendered

    html! {
        <button onclick={on_button_click}>{ "Send Data to Parent" }</button>
    }
}

#[function_component]
pub fn RenderChild(prop: &ChildProps) -> Html {
    prop.on_data_send.emit("Child on render...".to_string());
    html! {
        <p>{ "child rendered" }</p>
    }
}

// Parent component that receives data from the Child and displays it
#[function_component]
fn Parent() -> Html {
    // Callback to handle data sent from the Child component
    let on_data = Callback::from(|data: String| {
        // Print the data received from the child to the console
        web_sys::console::log_1(&data.into());
        // In a real-world scenario, you might update state here instead
    });
    // parent has the callback implemented
    // in its body, and that is assigned to Child Prop
    html! {
        <div>
            // Render the Child component,
        // passing the callback as a prop
            <Child on_data_send={&on_data} />
            <p>{ "Parent Receptor Component" }</p>
        // there is another child
            <RenderChild on_data_send={&on_data} />
        </div>
    }
}

// Main application component
#[function_component]
pub fn App() -> Html {
    html! {
        <Parent />
    }
}
