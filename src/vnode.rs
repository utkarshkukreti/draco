use crate::{Lazy, Mailbox, VKeyedElement, VNonKeyedElement, VText};
use derivative::Derivative;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub enum VNode<Message: 'static> {
    Element(VNonKeyedElement<Message>),
    KeyedElement(VKeyedElement<Message>),
    Text(VText),
    Lazy(Lazy<Message>),
}

impl<Message: 'static> VNode<Message> {
    pub fn create(&mut self, mailbox: &Mailbox<Message>) -> web::Node {
        let node = match self {
            VNode::Element(element) => element.create(mailbox).into(),
            VNode::KeyedElement(keyed_element) => keyed_element.create(mailbox).into(),
            VNode::Text(text) => text.create().into(),
            VNode::Lazy(lazy) => lazy.create(mailbox),
        };

        self.did_create(&node, mailbox);

        node
    }

    pub fn did_create(&self, node: &web::Node, mailbox: &Mailbox<Message>) {
        match self {
            VNode::Element(element) => element.did_create(node, mailbox),
            VNode::KeyedElement(keyed_element) => keyed_element.did_create(node, mailbox),
            VNode::Text(_) => {}
            VNode::Lazy(lazy) => lazy.did_create(node, mailbox),
        }
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<Message>) -> web::Node {
        match (self, old) {
            (VNode::Element(ref mut e1), VNode::Element(ref mut e2)) if e1.name == e2.name => {
                e1.patch(e2, mailbox).into()
            }
            (VNode::KeyedElement(ref mut e1), VNode::KeyedElement(ref mut e2))
                if e1.name == e2.name =>
            {
                e1.patch(e2, mailbox).into()
            }
            (VNode::Text(ref mut t1), VNode::Text(ref mut t2)) => t1.patch(t2).into(),
            (VNode::Lazy(ref mut l1), VNode::Lazy(ref mut l2)) => l1.patch(l2, mailbox),
            (self_, old) => {
                let old_node = old.node().unwrap_throw();
                let parent_node = old_node.parent_node().unwrap_throw();
                let node = self_.create(mailbox);
                parent_node.replace_child(&node, &old_node).unwrap_throw();
                old.did_remove(mailbox);
                node
            }
        }
    }

    pub fn node(&self) -> Option<web::Node> {
        match self {
            VNode::Element(element) => element.node().map(Into::into),
            VNode::KeyedElement(keyed_element) => keyed_element.node().map(Into::into),
            VNode::Text(text) => text.node().map(Into::into),
            VNode::Lazy(lazy) => lazy.node(),
        }
    }

    pub fn remove(&self, mailbox: &Mailbox<Message>) {
        if let Some(node) = self.node() {
            if let Some(parent_node) = node.parent_node() {
                parent_node.remove_child(&node).unwrap_throw();
            }
        }
        self.did_remove(mailbox);
    }

    pub fn did_remove(&self, mailbox: &Mailbox<Message>) {
        match self {
            VNode::Element(element) => element.did_remove(mailbox),
            VNode::KeyedElement(keyed_element) => keyed_element.did_remove(mailbox),
            VNode::Text(_) => {}
            VNode::Lazy(lazy) => lazy.did_remove(mailbox),
        }
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(Message) -> NewMessage + 'static,
    ) -> VNode<NewMessage> {
        self.do_map(Rc::new(f))
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> VNode<NewMessage> {
        match self {
            VNode::Element(element) => VNode::Element(element.do_map(f)),
            VNode::KeyedElement(keyed_element) => VNode::KeyedElement(keyed_element.do_map(f)),
            VNode::Text(text) => VNode::Text(text),
            VNode::Lazy(lazy) => VNode::Lazy(lazy.do_map(f)),
        }
    }
}

impl<Message> From<VText> for VNode<Message> {
    fn from(text: VText) -> Self {
        VNode::Text(text)
    }
}

impl<Message: 'static> From<VNonKeyedElement<Message>> for VNode<Message> {
    fn from(element: VNonKeyedElement<Message>) -> Self {
        VNode::Element(element)
    }
}

impl<Message: 'static> From<VKeyedElement<Message>> for VNode<Message> {
    fn from(keyed_element: VKeyedElement<Message>) -> Self {
        VNode::KeyedElement(keyed_element)
    }
}

impl<Message: 'static> From<Lazy<Message>> for VNode<Message> {
    fn from(lazy: Lazy<Message>) -> Self {
        VNode::Lazy(lazy)
    }
}

impl<Message> From<&'static str> for VNode<Message> {
    fn from(str: &'static str) -> Self {
        VText::new(str).into()
    }
}

impl<Message> From<String> for VNode<Message> {
    fn from(string: String) -> Self {
        VText::new(string).into()
    }
}

macro_rules! from_to_string {
    ($($ty:ty)*) => {
        $(
            impl<Message> From<$ty> for VNode<Message> {
                fn from(t: $ty) -> Self {
                    VText::new(t.to_string()).into()
                }
            }
        )*
    };
}

from_to_string! {
    bool
    char
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
}
