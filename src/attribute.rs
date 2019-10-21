use crate::S;
use wasm_bindgen::UnwrapThrowExt;
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
                .set_attribute(
                    wasm_bindgen::intern(&self.name),
                    wasm_bindgen::intern(&self.value),
                )
                .unwrap_throw()
        }
    }

    pub fn remove(&self, element: &web::Element) {
        element.remove_attribute(&self.name).unwrap_throw();
    }
}
