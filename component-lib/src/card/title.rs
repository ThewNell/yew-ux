use super::style::CardClass;
use yew::{html, Component};

pub struct CardTitle;

impl Component for CardTitle {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <h5 class={ CardClass::CardTitle }>
                { "Special Title Treatment" }
            </h5>
        }
    }
}
