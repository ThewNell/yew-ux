use super::style::FormClass;
use yew::{classes, html, Children, Component, Context, Html, Properties};

pub struct FormGroup;

#[derive(PartialEq, Properties)]
pub struct FormGroupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub hidden: bool,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden = ctx.props().hidden;
        if hidden {
            return Html::default();
        }

        let class = ctx.props().class.clone();
        let classes = classes!(class, FormClass::FormGroup.to_string());

        html! {
            <div class={ classes }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
