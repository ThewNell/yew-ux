use super::style::FormClass;
use yew::{html, Children, Component, Properties};

pub struct FormGroup;

#[derive(PartialEq, Properties)]
pub struct FormGroupProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ FormClass::FormGroup }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
