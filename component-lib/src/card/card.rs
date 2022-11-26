use super::style::CardClass;
use yew::{html, Children, Component, Properties};

pub struct Card;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ CardClass::Card }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
