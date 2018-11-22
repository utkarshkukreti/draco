use crate::S;
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
            .expect("window")
            .document()
            .expect("document")
            .create_text_node(&self.value);
        self.node = Some(node.clone());
        node
    }

    pub fn patch(&mut self, old: &mut Text) -> web::Text {
        let node = old.node.take().expect("old.node");
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
