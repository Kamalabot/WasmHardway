use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextProps {
    pub message: String,
}
// text props just fill in para or headers
#[derive(Properties, PartialEq)]
pub struct ColorProp {
    pub color: String,
    pub your_text: String,
}
#[function_component]
fn DisplayText(prop: &TextProps) -> Html {
    // need to use the div, p
    html! {
        <div>
        <p>{ &prop.message }</p>
        </div>
    }
}

#[function_component]
pub fn ColoredBox(prop: &ColorProp) -> Html {
    html! {
        <div style={format!("background-color: {};", prop.color)}>{prop.your_text.clone()}</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ImageProp {
    pub url: String,
}

#[function_component]
pub fn ImageBox(prop: &ImageProp) -> Html {
    html! {
        <img src={prop.url.clone()} alt="Dynamic Image" width="150" height="150" />
    }
}
#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub items: Vec<String>,
}

#[function_component]
pub fn ListDisplay(prop: &ListProps) -> Html {
    html! {
        <ul>
            { for prop.items.iter().map(|item| html! { <li>{item}</li> })}
        </ul>
    }
}
#[derive(Properties, PartialEq)]
pub struct BoolProp {
    pub is_visible: bool,
}

#[function_component]
pub fn ToggleDis(prop: &BoolProp) -> Html {
    if prop.is_visible {
        html! { <p>{" Visible Content "}</p> }
    } else {
        html! { <p>{" InVisible Content "}</p> }
    }
}

#[function_component]
pub fn App() -> Html {
    let text1: Html = html! { <DisplayText message="Hey there from prop " /> };
    let text2: Html = html! { <DisplayText message="Another text prop" /> };
    let colbox1: Html = html! { <ColoredBox color="blue" your_text="Blue me" />};
    let colbox2: Html = html! { <ColoredBox color="brown" your_text="Brown me" />};
    let imgbox: Html = html! { <ImageBox url="/static/euro.jpg" />};
    let imgbox1: Html = html! { <ImageBox url="https://pixabay.com/get/g730dacbf99ea7a174476e7292f32dc4351b0c711c0ee9fc2d718f4a9cdfcff6c7b6863372c49350c422882d74868c3d5c15f17bad132d45dc8685bc04cc3b1e9b2ce4ab69cc6ec278a58ac76ad95af61_640.jpg" />};
    let your_items: Vec<String> = vec!["item1".to_owned(), "item2".to_owned(), "item3".to_owned()];
    let lister = html! { <ListDisplay items={your_items} />};
    let vis1 = html! { <ToggleDis is_visible={true} /> };
    let vis2 = html! { <ToggleDis is_visible={false} /> };
    // so text prop simply places the text on fe
    html! {
        <div>
        {text1}
        {text2}
        {colbox1}
        {colbox2}
        {imgbox}
        {imgbox1}
        {lister}
        {vis1}
        {vis2}
        </div>
    }
}
