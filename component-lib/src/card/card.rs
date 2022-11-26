use super::style::CardClass;
use yew::{classes, html, Children, Component, Properties};

pub struct Card;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardProps {
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
        let mut styles = vec![CardClass::Card];
        if ctx.props().hidden {
            styles.push(CardClass::Hidden);
        }

        let styles: Vec<String> =
            styles.iter().map(|style| style.to_string()).collect();

        let classes = classes!(styles);

        html! {
            <div class={ classes }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
