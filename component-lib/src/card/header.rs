use super::style::CardClass;
use yew::{html, Children, Component, Properties};

pub struct CardHeader;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for CardHeader {
    type Message = ();
    type Properties = CardHeaderProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ CardClass::CardHeader }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
