use super::style::CardClass;
use yew::{classes, html, Children, Component, Html, Properties};

pub struct Card;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub hidden: bool,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let hidden = ctx.props().hidden;
        if hidden {
            return Html::default();
        }

        let class = ctx.props().class.clone();
        let classes = classes!(class, CardClass::Card);

        html! {
            <div class={ classes }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
