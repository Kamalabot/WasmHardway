#![allow(unused_imports)]
#![allow(warnings)]

use web_sys::{console, HtmlElement, MouseEvent};
use yew::prelude::*;
use yew::{classes, html, Callback, Classes, TargetCast};

#[function_component]
pub fn App() -> Html {
    // inside the app, you basically write the
    // html tags, and then get them rendered
    // by returnning them as a single html
    // We can work on target htmlelements using
    // the websys
    let imager: Html = html! {
         <img src="new_img.jpg" alt="boy in bucket" width="500" height="300"/>
    };
    let header_txt = "Hey there".to_owned();
    let new_head: Html = html! {
        <h1>{header_txt}</h1>
    };
    let count: usize = 5;
    let count_age: Html = html! {
        <p>{"My age is: "}{count}</p>
    };

    // using fragments
    let frags = html! {
        <>
        <div><p>{"Para in div"}</p></div>
        <p>{"Frag para"}</p>
        </>
    };
    // exploring classes
    let cl1 = html! {
    <div class={classes!("tonter")}><p>{"Class here."}</p></div>
    };
    // exploring classes
    let cl2 = html! {
    <div class={classes!("tont1", "tont2")}><p>{"Class more."}</p></div>
    };
    use yew::{classes, html};

    let clop3 = html! {
      <div class={classes!(Some("test1"))} />
    };
    let in_ls = html! {
      <div style="color: blue; bold: true"><p>{"Make Nice & Red"}</p></div>
    };
    let onmousemove = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_y() as f64) - rect.top();
            console::log_1(&format!("left? :{} Top?: {}", x, y).into())
        }
    });
    let movetml = html! {
        <div id="mousemoveme" {onmousemove}>{"Here is one mouse move"}</div>
    };
    let new_header: Html = html! {
        <>
        <div>{new_head}{count_age}{imager}{frags}</div>
        <div>{cl1}</div>
        <div>{cl2}</div>
        <div>{clop3}</div>
        <div>{in_ls}</div>
        {movetml}
        </>
    };
    new_header
}
