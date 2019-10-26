use crate::S;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

#[derive(Debug)]
pub struct VText {
    value: S,
    node: Option<web::Text>,
}

impl VText {
    pub fn new<V: Into<S>>(value: V) -> Self {
        VText {
            value: value.into(),
            node: None,
        }
    }

    pub fn create(&mut self) -> web::Text {
        let node = web::window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .create_text_node(&self.value);
        self.node = Some(node.clone());
        node
    }

    pub fn patch(&mut self, old: &mut VText) -> web::Text {
        let node = old.node.clone().unwrap_throw();
        if self.value != old.value {
            node.set_data(&self.value);
        }
        self.node = Some(node.clone());
        node
    }

    pub fn node(&self) -> Option<web::Text> {
        self.node.clone()
    }
}
