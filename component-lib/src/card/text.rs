use super::style::CardClass;
use yew::{html, Component};

pub struct CardText;

impl Component for CardText {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <p class={ CardClass::CardText }>
                { "Special Text Treatment" }
            </p>
        }
    }
}
