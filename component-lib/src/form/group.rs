use super::style::FormClass;
use yew::{classes, html, Children, Component, Properties};

pub struct FormGroup;

#[derive(PartialEq, Properties)]
pub struct FormGroupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub hidden: bool,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut styles = vec![FormClass::FormGroup];
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
