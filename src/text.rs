use crate::S;
use std::cell::RefCell;
use web_sys as web;

#[derive(Debug)]
pub struct Text {
    value: S,
    node: RefCell<Option<web::Text>>,
}

impl Text {
    pub fn new<V: Into<S>>(value: V) -> Self {
        Text {
            value: value.into(),
            node: RefCell::new(None),
        }
    }

    pub fn create(&self) -> web::Text {
        let node = web::window()
            .expect("window")
            .document()
            .expect("document")
            .create_text_node(&self.value);
        self.node.replace(Some(node.clone()));
        node
    }

    pub fn patch(&self, old: &Text) -> web::Text {
        let node = old.node.replace(None).expect("old.node");
        if self.value != old.value {
            (node.as_ref() as &web::Node).set_text_content(Some(&self.value));
        }
        self.node.replace(Some(node.clone()));
        node
    }

    pub fn node(&self) -> Option<web::Text> {
        self.node.borrow().clone()
    }
}
