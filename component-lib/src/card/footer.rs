use super::style::CardClass;
use yew::{html, Children, Component, Properties};

pub struct CardFooter;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ CardClass::CardFooter }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
