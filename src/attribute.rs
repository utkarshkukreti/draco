use crate::S;
use web_sys as web;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: S,
    pub value: S,
}

impl Attribute {
    pub fn patch(&self, old_attribute: Option<&Attribute>, element: &web::Element) {
        if Some(self) != old_attribute {
            element
                .set_attribute(&self.name, &self.value)
                .expect("set_attribute")
        }
    }
}
