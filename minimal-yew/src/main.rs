mod app;

use app::Dpp;

fn main() {
    yew::Renderer::<Dpp>::new().render();
    // starts appln and mounts it to page's body tag
    // yew::Renderer::<Dpp>::with_props().render();
    // for dynamic property
}
