use component_lib::form::FormGroup;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <form>
            <FormGroup>
                { "Form Group Content" }
            </FormGroup>
        </form>
    }
}
