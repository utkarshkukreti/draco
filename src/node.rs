use crate::{KeyedElement, Mailbox, NonKeyedElement, Text};
use std::rc::Rc;
use web_sys as web;

#[derive(Debug)]
pub enum Node<Message: 'static> {
    Element(NonKeyedElement<Message>),
    KeyedElement(KeyedElement<Message>),
    Text(Text),
}

impl<Message: 'static> Node<Message> {
    pub fn create(&mut self, mailbox: &Mailbox<Message>) -> web::Node {
        match self {
            Node::Element(element) => element.create(mailbox).into(),
            Node::KeyedElement(keyed_element) => keyed_element.create(mailbox).into(),
            Node::Text(text) => text.create().into(),
        }
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<Message>) -> web::Node {
        match (self, old) {
            (Node::Element(ref mut e1), Node::Element(ref mut e2)) => e1.patch(e2, mailbox).into(),
            (Node::KeyedElement(ref mut e1), Node::KeyedElement(ref mut e2)) => {
                e1.patch(e2, mailbox).into()
            }
            (Node::Text(ref mut t1), Node::Text(ref mut t2)) => t1.patch(t2).into(),
            (self_, old) => {
                let old_node = old.node().expect("old.node");
                let parent_node = old_node.parent_node().expect("old_node.parent_node");
                let node = self_.create(mailbox);
                parent_node
                    .replace_child(&node, &old_node)
                    .expect("replace_child");
                node
            }
        }
    }

    pub fn node(&self) -> Option<web::Node> {
        match self {
            Node::Element(element) => element.node().map(Into::into),
            Node::KeyedElement(keyed_element) => keyed_element.node().map(Into::into),
            Node::Text(text) => text.node().map(Into::into),
        }
    }

    pub fn remove(&self) {
        if let Some(node) = self.node() {
            if let Some(parent_node) = node.parent_node() {
                parent_node.remove_child(&node).unwrap();
            }
        }
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(Message) -> NewMessage + 'static,
    ) -> Node<NewMessage> {
        self.do_map(Rc::new(f))
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> Node<NewMessage> {
        match self {
            Node::Element(element) => Node::Element(element.do_map(f)),
            Node::KeyedElement(keyed_element) => Node::KeyedElement(keyed_element.do_map(f)),
            Node::Text(text) => Node::Text(text),
        }
    }
}

impl<Message> From<Text> for Node<Message> {
    fn from(text: Text) -> Self {
        Node::Text(text)
    }
}

impl<Message: 'static> From<NonKeyedElement<Message>> for Node<Message> {
    fn from(element: NonKeyedElement<Message>) -> Self {
        Node::Element(element)
    }
}

impl<Message: 'static> From<KeyedElement<Message>> for Node<Message> {
    fn from(keyed_element: KeyedElement<Message>) -> Self {
        Node::KeyedElement(keyed_element)
    }
}

impl<Message> From<&'static str> for Node<Message> {
    fn from(str: &'static str) -> Self {
        Text::new(str).into()
    }
}

impl<Message> From<String> for Node<Message> {
    fn from(string: String) -> Self {
        Text::new(string).into()
    }
}

macro_rules! from_to_string {
    ($($ty:ty)*) => {
        $(
            impl<Message> From<$ty> for Node<Message> {
                fn from(t: $ty) -> Self {
                    Text::new(t.to_string()).into()
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
