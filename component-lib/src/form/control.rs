use super::style::FormClass;
use std::fmt::Display;
use yew::{html, Component, Properties};

pub struct FormControl;

#[derive(PartialEq, Properties)]
pub struct FormControlProps {
    pub id: String,
    #[prop_or_default]
    pub input: FormControlType,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum FormControlType {
    Button,
    CheckBox,
    Color,
    Date,
    DateTimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Telephone,
    #[default]
    Text,
    Time,
    Url,
    Week,
}

impl Display for FormControlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FormControlType {
    fn as_string(&self) -> String {
        match self {
            FormControlType::DateTimeLocal => "datetime-local".to_owned(),
            FormControlType::Telephone => "tel".to_owned(),
            _ => self.to_string().to_lowercase(),
        }
    }
}

// tight tense normal stretchy loose

impl Component for FormControl {
    type Message = ();
    type Properties = FormControlProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let id = ctx.props().id.clone();
        let input = ctx.props().input.clone();
        let label = ctx.props().label.clone();
        let name = ctx.props().name.clone();
        let placeholder = ctx.props().placeholder.clone();
        let value = ctx.props().value.clone();

        let label_id = format!("{}Label", id.clone());

        html! {
            <>
                if label.len() > 0 {
                    <label id={ label_id.clone() } for={ id.clone() }>
                        { label.clone() }
                    </label>
                }

                if let FormControlType::Button = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } value={ value } />
                } else if let FormControlType::Color = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } name={ name } value={ value } />
                } else if let FormControlType::Date = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } name={ name } />
                } else if let FormControlType::DateTimeLocal = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } name={ name } />
                } else if let FormControlType::Email = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } placeholder={ placeholder } value={ value } />
                } else if let FormControlType::Text = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } placeholder={ placeholder } value={ value } />
                } else if let FormControlType::Password = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } placeholder={ placeholder } value={ value } />
                } else if let FormControlType::Submit = input {
                    <input id={ id } type={ input.as_string() } class={ FormClass::FormControl } value={ value } />
                }
            </>
        }
    }
}
