use super::style::CardClass;
use yew::{html, Children, Component, Properties};

pub struct CardBody;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for CardBody {
    type Message = ();
    type Properties = CardBodyProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ CardClass::CardBody }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
