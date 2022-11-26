use yew::{classes, html, Children, Component, Properties};

use super::style::FormClass;

pub struct FormCheck;

#[derive(PartialEq, Properties)]
pub struct FormCheckProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub hidden: bool,
}

impl Component for FormCheck {
    type Message = ();
    type Properties = FormCheckProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut styles = vec![FormClass::FormCheck];
        if ctx.props().hidden {
            styles.push(FormClass::Hidden);
        }

        let classes = classes!(styles);

        html! {
            <div class={ classes }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
