use std::fmt::Display;

use yew::{Classes, classes};

pub enum FormClass {
    FormCheck,
    FormControl,
    FormGroup,
}

impl Display for FormClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl FormClass {
    fn as_string(&self) -> String {
        match self {
            FormClass::FormCheck => "form-check".to_owned(),
            FormClass::FormControl => "form-control".to_owned(),
            FormClass::FormGroup => "form-group".to_owned(),            
        }
    }
}

impl Into<Classes> for FormClass {
    fn into(self) -> Classes {
        classes!(self.to_string())
    }
}