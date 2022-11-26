use yew::{html, Component};

use super::props::{
    CardBodyProps, CardFooterProps, CardHeaderProps, CardProps,
};
use super::style::CardClass;

pub struct Card;

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ CardClass::Card }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}

pub struct CardBody;

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

pub struct CardTitle;

impl Component for CardTitle {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <h5 class={ CardClass::CardTitle }>
                { "Special Title Treatment" }
            </h5>
        }
    }
}

pub struct CardText;

impl Component for CardText {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <p class={ CardClass::CardText }>
                { "Special Text Treatment" }
            </p>
        }
    }
}

pub struct CardFooter;

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class={ CardClass::CardFooter }>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}

pub struct CardHeader;

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
