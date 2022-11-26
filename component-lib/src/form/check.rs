use yew::{classes, html, Children, Component, Context, Html, Properties};

use super::style::FormClass;

pub struct FormCheck;

#[derive(PartialEq, Properties)]
pub struct FormCheckProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub hidden: bool,
}

impl Component for FormCheck {
    type Message = ();
    type Properties = FormCheckProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden = ctx.props().hidden;
        if hidden {
            return Html::default();
        }

        let class = ctx.props().class.clone();
        let classes = classes!(class, FormClass::FormCheck);

        html! {
            <div class={ classes }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
