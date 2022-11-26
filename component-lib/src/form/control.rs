use super::style::FormClass;
use std::fmt::Display;
use yew::{html, AttrValue, Component, Context, Html, Properties};

pub struct FormControl;

#[derive(PartialEq, Properties)]
pub struct FormControlProps {
    #[prop_or_default]
    pub alternate: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub height: u32,
    pub id: String,
    #[prop_or_default]
    pub input: FormControlType,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub max: Option<f32>,
    #[prop_or_default]
    pub min: Option<f32>,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub pattern: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub source: String,
    #[prop_or_default]
    pub step: Option<f32>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub width: u32,
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
    alternate: String,
    disabled: bool,
    height: u32,
    id: String,
    input: FormControlType,
    max: Option<f32>,
    min: Option<f32>,
    multiple: bool,
    name: String,
    placeholder: String,
    pattern: String,
    readonly: bool,
    source: String,
    step: Option<f32>,
    title: String,
    value: String,
    width: u32,
}

impl FormControlBuilder {
    pub fn alternate(mut self, alternate: String) -> Self {
        self.alternate = alternate;
        self
    }

    pub fn build(self) -> Html {
        let mut input_attrs = vec![
            "<input".to_owned(),
            format!("id={}", self.id),
            format!("type={}", self.input.as_string()),
            format!("class={}", FormClass::FormControl),
        ];

        // empty attributes
        if self.disabled || self.readonly {
            input_attrs.push("disabled".to_owned());
            input_attrs.push("readonly".to_owned());
        }

        if self.multiple {
            input_attrs.push("multiple".to_owned());
        }

        // value attributes
        if self.alternate.len() > 0 {
            let alt_attr = format!("alt={}", self.alternate);
            input_attrs.push(alt_attr);
        }

        if self.height > 0 {
            let height_attr = format!("height={:?}", self.height);
            input_attrs.push(height_attr);
        }

        if self.max.is_some() {
            let max_attr = format!("max={:?}", self.max.unwrap_or_default());
            input_attrs.push(max_attr);
        }

        if self.min.is_some() {
            let min_attr = format!("min={:?}", self.min.unwrap_or_default());
            input_attrs.push(min_attr);
        }

        if self.name.len() > 0 {
            let name_attr = format!("name={:?}", self.name);
            input_attrs.push(name_attr);
        }

        if self.pattern.len() > 0 {
            let pattern_attr = format!("pattern={:?}", self.pattern);
            input_attrs.push(pattern_attr);
        }

        if self.placeholder.len() > 0 {
            let placeholder_attr =
                format!("placeholder={:?}", self.placeholder);
            input_attrs.push(placeholder_attr);
        }

        if self.source.len() > 0 {
            let source_attr = format!("src={:?}", self.source);
            input_attrs.push(source_attr);
        }

        if self.step.is_some() {
            let step_attr = format!("step={:?}", self.step.unwrap_or_default());
            input_attrs.push(step_attr);
        }

        if self.title.len() > 0 {
            let title_attr = format!("title={:?}", self.title);
            input_attrs.push(title_attr);
        }

        if self.value.len() > 0 {
            let value_attr = format!("value={:?}", self.value);
            input_attrs.push(value_attr);
        }

        if self.width > 0 {
            let width_attr = format!("width={:?}", self.width);
            input_attrs.push(width_attr);
        }

        input_attrs.push("/>".to_owned());
        let input_string = input_attrs.join(" ");

        Html::from_html_unchecked(AttrValue::from(input_string))
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn input(mut self, input: FormControlType) -> Self {
        self.input = input;
        self
    }

    pub fn max(mut self, max: Option<f32>) -> Self {
        self.max = max;
        self
    }

    pub fn min(mut self, min: Option<f32>) -> Self {
        self.min = min;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
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

    pub fn pattern(mut self, pattern: String) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    pub fn step(mut self, step: Option<f32>) -> Self {
        self.step = step;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
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
        let alternate = ctx.props().alternate.clone();
        let disabled = ctx.props().disabled;
        let height = ctx.props().height;
        let id = ctx.props().id.clone();
        let input = ctx.props().input.clone();
        let label = ctx.props().label.clone();
        let max = ctx.props().max;
        let min = ctx.props().min;
        let multiple = ctx.props().multiple;
        let name = ctx.props().name.clone();
        let pattern = ctx.props().pattern.clone();
        let placeholder = ctx.props().placeholder.clone();
        let readonly = ctx.props().readonly;
        let source = ctx.props().source.clone();
        let step = ctx.props().step;
        let title = ctx.props().title.clone();
        let value = ctx.props().value.clone();
        let width = ctx.props().width;

        let label_id = format!("{}Label", id.clone());
        let input_html = FormControlBuilder::new(id.clone())
            .alternate(alternate)
            .disabled(disabled)
            .height(height)
            .input(input)
            .max(max)
            .min(min)
            .multiple(multiple)
            .name(name)
            .pattern(pattern)
            .placeholder(placeholder)
            .readonly(readonly)
            .source(source)
            .step(step)
            .title(title)
            .value(value)
            .width(width)
            .build();

        html! {
            <>
                if label.len() > 0 {
                    <label class={ FormClass::FormLabel } id={ label_id } for={ id }>
                        { label.clone() }
                    </label>
                }

                { input_html }
            </>
        }
    }
}
