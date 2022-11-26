use yew::{Children, Properties};

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, Default, PartialEq, Properties)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
}
