use super::style::FormClass;
use std::fmt::Display;
use yew::{html, Component, Properties, Context, Html, AttrValue};

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

#[derive(Clone, Debug, Default)]
pub struct FormControlBuilder {
    id: String,
    input: FormControlType,
    name: String,
    placeholder: String,
    value: String,
}

impl FormControlBuilder {
    pub fn build(self) -> Html {
        let mut input_attrs = vec![
            "<input".to_owned(),
            format!("class={}", FormClass::FormControl),
            format!("id={}", self.id),
        ];

        if self.name.len() > 0 {
            let name_attr = format!("name={}", self.name);
            input_attrs.push(name_attr);
        }

        if self.placeholder.len() > 0 {
            let placeholder_attr = format!("placeholder={}", self.placeholder);
            input_attrs.push(placeholder_attr);
        }
        
        input_attrs.push(format!("type={}", self.input.as_string()));

        if self.value.len() > 0 {
            let value_attr = format!("value={}", self.value);
            input_attrs.push(value_attr);
        }

        input_attrs.push("/>".to_owned());
        let input_string = input_attrs.join(" ");

        Html::from_html_unchecked(AttrValue::from(input_string))
    }

    pub fn input(mut self, input: FormControlType) -> Self {
        self.input = input;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }
}

// tight tense normal stretchy loose

impl Component for FormControl {
    type Message = ();
    type Properties = FormControlProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();
        let input = ctx.props().input.clone();
        let label = ctx.props().label.clone();
        let name = ctx.props().name.clone();
        let placeholder = ctx.props().placeholder.clone();
        let value = ctx.props().value.clone();

        let label_id = format!("{}Label", id.clone());

        let input_html = FormControlBuilder::new(id.clone())
            .input(input)
            .name(name)
            .placeholder(placeholder)
            .value(value)
            .build();

        html! {
            <>
                if label.len() > 0 {
                    <label id={ label_id } for={ id }>
                        { label.clone() }
                    </label>
                }
                
                { input_html }
            </>
        }
    }
}
