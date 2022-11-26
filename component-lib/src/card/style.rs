use std::fmt::{Display, Formatter, self};
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_string())
    }
}

impl Into<Classes> for CardClass {
    fn into(self) -> Classes {
        classes!(self.to_string())
    }
}

impl CardClass {
    fn as_string(&self) -> String {
        match self {
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
}
