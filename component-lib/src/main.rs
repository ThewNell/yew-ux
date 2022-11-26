use yew::prelude::*;
use crate::card::{
    body::CardBody, card::Card, footer::CardFooter, header::CardHeader,
    text::CardText, title::CardTitle,
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
                </CardBody>
                <CardFooter />
            </Card>
        </div>
    }
}
