#![allow(unused_imports)]

use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct CounterState {
    count: i32,
}

// impl CounterState {
//     pub fn new() -> Rc<Self> {
//         Rc::new(CounterState { count: 0 })
//     }
// }

impl Reducible for CounterState {
    type Action = CounterAction;

    fn reduce(self: Rc<Self>, action: CounterAction) -> Rc<Self> {
        match action {
            CounterAction::Increment => Rc::new(CounterState {
                count: self.count + 1,
            }),
            CounterAction::Decrement => Rc::new(CounterState {
                count: self.count - 1,
            }),
        }
    }
}
// simple pub enum
pub enum CounterAction {
    Increment,
    Decrement,
}

pub fn Counter() -> Html {
    // let state = use_reducer(|| Rc::new(CounterState { count: 0 }), reducer);
    let state = use_reducer(|| CounterState { count: 0 });

    let inc = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(CounterAction::Increment))
    };
    html! {
        <div>
            <button onclick={state.dispatch(CounterAction::Decrement)}>{" Decrement" }</button>
            <p>{ state.count }</p>
            <button onclick={state.dispatch(CounterAction::Increment)}>{" Increment "}</button>
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
