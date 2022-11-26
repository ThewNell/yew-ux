use yew::prelude::*;
use crate::card::comp::{
    Card, CardBody, CardFooter, CardHeader, CardText, CardTitle,
};

mod card;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div class={ "container" }>
            <Card>
                <CardHeader />
                <CardBody>
                    <CardTitle />
                    <CardText />
                    <a href={ "#" } class={ "btn btn-primary" }>
                        { "Button" }
                    </a>
                </CardBody>
                <CardFooter>
                    { "Some Footer Content" }
                </CardFooter>
            </Card>
        </div>
    }
}
