use std::fmt::Display;
use yew::{classes, Classes};

pub enum CardClass {
    Card,

    /// Used for a padded section within a card (use on \<div> tags).
    CardBody,
    CardFooter,
    CardHeader,

    /// Used to place an image to the top of the card (use on \<img> tags).
    CardImgTop,

    /// Used on \<a> tags.
    CardLink,

    /// Used on \<h*> tags.
    CardSubtitle,
    CardText,

    /// Used on \<h*> tags.
    CardTitle,
}

impl Display for CardClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", get_card_class(self))
    }
}

impl Into<Classes> for CardClass {
    fn into(self) -> Classes {
        classes!(self.to_string())
    }
}

fn get_card_class(class: &CardClass) -> String {
    match class {
        CardClass::Card => "card".to_owned(),
        CardClass::CardBody => "card-body".to_owned(),
        CardClass::CardFooter => "card-footer".to_owned(),
        CardClass::CardHeader => "card-header".to_owned(),
        CardClass::CardImgTop => "card-img-top".to_owned(),
        CardClass::CardLink => "card-link".to_string(),
        CardClass::CardSubtitle => "card-subtitle".to_owned(),
        CardClass::CardText => "card-text".to_owned(),
        CardClass::CardTitle => "card-title".to_owned(),
    }
}
