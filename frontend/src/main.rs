mod api;

use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Rust and Yew.rs Frontend App: RS256 JWT Access and Refresh Tokens"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}