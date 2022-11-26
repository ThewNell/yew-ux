use yew::{html, Children, Component, Properties};

use super::style::FormClass;

pub struct FormCheck;

#[derive(PartialEq, Properties)]
pub struct FormCheckProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for FormCheck {
    type Message = ();
    type Properties = FormCheckProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ FormClass::FormCheck }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
