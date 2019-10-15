use crate::S;
use std::borrow::Cow;
use wasm_bindgen::JsCast;
use web_sys as web;

#[derive(Debug)]
pub struct Attr {
    pub name: S,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(S),
    Bool(bool),
}

impl Attr {
    pub fn patch(&self, old_value: Option<&Value>, element: &web::Element) {
        match (&*self.name, &self.value) {
            ("checked", Value::Bool(checked)) => {
                if let Some(input) = element.dyn_ref::<web::HtmlInputElement>() {
                    if input.checked() != *checked {
                        input.set_checked(*checked);
                    }
                    return;
                }
            }
            ("value", Value::String(value)) => {
                if let Some(input) = element.dyn_ref::<web::HtmlInputElement>() {
                    if &input.value() != value {
                        input.set_value(&value);
                    }
                    return;
                }
                if let Some(textarea) = element.dyn_ref::<web::HtmlTextAreaElement>() {
                    if &textarea.value() != value {
                        textarea.set_value(&value);
                    }
                    return;
                }
            }
            _ => {}
        }
        if Some(&self.value) != old_value {
            match &self.value {
                Value::String(value) => element
                    .set_attribute(&self.name, &value)
                    .expect("set_attribute"),
                Value::Bool(true) => element
                    .set_attribute(&self.name, "")
                    .expect("set_attribute"),
                Value::Bool(false) => element
                    .remove_attribute(&self.name)
                    .expect("remove_attribute"),
            }
        }
    }
}

impl From<&'static str> for Value {
    fn from(str: &'static str) -> Self {
        Value::String(str.into())
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string.into())
    }
}

impl From<Cow<'static, str>> for Value {
    fn from(s: Cow<'static, str>) -> Self {
        Value::String(s)
    }
}

impl From<bool> for Value {
    fn from(bool: bool) -> Self {
        Value::Bool(bool)
    }
}
