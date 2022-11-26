use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <h1>{ "Hello World!" }</h1>
    }
}
