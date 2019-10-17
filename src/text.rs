use crate::S;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

#[derive(Debug)]
pub struct Text {
    value: S,
    node: Option<web::Text>,
}

impl Text {
    pub fn new<V: Into<S>>(value: V) -> Self {
        Text {
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

    pub fn patch(&mut self, old: &mut Text) -> web::Text {
        let node = old.node.take().unwrap_throw();
        if self.value != old.value {
            (node.as_ref() as &web::Node).set_text_content(Some(&self.value));
        }
        self.node = Some(node.clone());
        node
    }

    pub fn node(&self) -> Option<web::Text> {
        self.node.clone()
    }
}
