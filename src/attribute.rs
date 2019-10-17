use crate::S;
use web_sys as web;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: S,
    value: S,
}

impl Attribute {
    pub fn new(name: impl Into<S>, value: impl Into<S>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn patch(&self, old_attribute: Option<&Attribute>, element: &web::Element) {
        if Some(self) != old_attribute {
            element
                .set_attribute(&self.name, &self.value)
                .expect("set_attribute")
        }
    }

    pub fn remove(&self, element: &web::Element) {
        element
            .remove_attribute(&self.name)
            .expect("remove_attribute");
    }
}
