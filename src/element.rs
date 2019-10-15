use crate::{attr, Attr, Listener, Mailbox, Node, S};
// use std::collections::HashMap;
use fxhash::FxHashMap as HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys as web;

pub type NonKeyedElement<Message> = Element<NonKeyed<Message>>;
pub type KeyedElement<Message> = Element<Keyed<Message>>;

#[derive(Debug)]
pub struct Element<C: Children> {
    name: &'static str,
    ns: Ns,
    attrs: Vec<Attr>,
    listeners: Vec<Listener<C::Message>>,
    children: C,
    node: Option<web::Element>,
}

#[derive(Debug)]
pub enum Ns {
    Html,
    Svg,
}

#[derive(Debug, Default)]
pub struct Keyed<Message: 'static>(Vec<(u64, Node<Message>)>);

#[derive(Debug, Default)]
pub struct NonKeyed<Message: 'static>(Vec<Node<Message>>);

pub fn h<Message: 'static>(name: &'static str) -> NonKeyedElement<Message> {
    Element::new(Ns::Html, name)
}

pub fn s<Message: 'static>(name: &'static str) -> NonKeyedElement<Message> {
    Element::new(Ns::Svg, name)
}

impl<C: Children> Element<C>
where
    C::Message: 'static,
{
    pub fn new(ns: Ns, name: &'static str) -> Self {
        Element {
            name,
            ns,
            attrs: Vec::new(),
            listeners: Vec::new(),
            children: C::new(),
            node: None,
        }
    }

    pub fn attr<N: Into<S>, V: Into<attr::Value>>(mut self, name: N, value: V) -> Self {
        self.attrs.push(Attr {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    pub fn on<N: Into<S>>(
        mut self,
        name: N,
        handler: impl FnMut(web::Event) -> C::Message + 'static,
    ) -> Self {
        self.listeners.push(Listener::new(name, handler));
        self
    }

    pub fn on_input(self, mut handler: impl FnMut(String) -> C::Message + 'static) -> Self {
        self.on("input", move |event| {
            if let Some(target) = event.target() {
                if let Some(input) = target.dyn_ref::<web::HtmlInputElement>() {
                    return handler(input.value());
                }
                if let Some(input) = target.dyn_ref::<web::HtmlTextAreaElement>() {
                    return handler(input.value());
                }
                if let Some(input) = target.dyn_ref::<web::HtmlSelectElement>() {
                    return handler(input.value());
                }
            }
            return handler("".into());
        })
    }

    pub fn on_checked(self, mut handler: impl FnMut(bool) -> C::Message + 'static) -> Self {
        self.on("input", move |event| {
            if let Some(target) = event.target() {
                if let Some(input) = target.dyn_ref::<web::HtmlInputElement>() {
                    return handler(input.checked());
                }
            }
            return handler(false);
        })
    }

    pub fn create(&mut self, mailbox: &Mailbox<C::Message>) -> web::Element {
        let document = web::window().expect("window").document().expect("document");

        let node = match self.ns {
            Ns::Html => document.create_element(&self.name).expect("create_element"),
            Ns::Svg => document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), &self.name)
                .expect("create_element_ns"),
        };

        for attr in &self.attrs {
            attr.patch(None, &node);
        }

        for listener in &mut self.listeners {
            listener.attach(&node, mailbox);
        }

        self.children.create(node.as_ref() as &web::Node, mailbox);

        self.node = Some(node.clone());
        node
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<C::Message>) -> web::Element {
        let old_node = old.node.take().expect("old.node");
        if self.name != old.name {
            let new_node = self.create(mailbox);
            (old_node.as_ref() as &web::Node)
                .parent_node()
                .expect("old_node.parent_node")
                .replace_child(new_node.as_ref(), old_node.as_ref())
                .expect("replace_child");
            return new_node;
        }

        for attr in &self.attrs {
            let old_attr = old
                .attrs
                .iter()
                .find(|old_attr| old_attr.name == attr.name)
                .map(|attr| &attr.value);
            attr.patch(old_attr, &old_node);
        }

        for old_attr in &old.attrs {
            if !self
                .attrs
                .iter()
                .any(|new_attr| new_attr.name == old_attr.name)
            {
                old_node
                    .remove_attribute(&old_attr.name)
                    .expect("remove_attribute");
            }
        }

        for listener in &old.listeners {
            listener.detach(&old_node);
        }

        for listener in &mut self.listeners {
            listener.attach(&old_node, mailbox);
        }

        self.children
            .patch(&mut old.children, old_node.as_ref(), mailbox);

        self.node = Some(old_node.clone());

        old_node
    }

    pub fn node(&self) -> Option<web::Element> {
        self.node.clone()
    }
}

impl<Message: 'static> NonKeyedElement<Message> {
    pub fn push<N: Into<Node<Message>>>(mut self, node: N) -> Self {
        self.children.0.push(node.into());
        self
    }

    pub fn append<N: Into<Node<Message>>, I: IntoIterator<Item = N>>(mut self, i: I) -> Self {
        self.children.0.extend(i.into_iter().map(Into::into));
        self
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(Message) -> NewMessage + 'static,
    ) -> NonKeyedElement<NewMessage> {
        self.do_map(Rc::new(f))
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> NonKeyedElement<NewMessage> {
        let Element {
            name,
            ns,
            attrs,
            listeners,
            children,
            node,
        } = self;
        let listeners = listeners
            .into_iter()
            .map(|listener| listener.do_map(f.clone()))
            .collect();
        let children = NonKeyed(
            children
                .0
                .into_iter()
                .map(|n| n.do_map(f.clone()))
                .collect(),
        );
        Element {
            name,
            ns,
            attrs,
            listeners,
            children,
            node,
        }
    }
}

impl<Message: 'static> KeyedElement<Message> {
    pub fn push<N: Into<Node<Message>>>(mut self, key: u64, node: N) -> Self {
        self.children.0.push((key, node.into()));
        self
    }

    pub fn append<N: Into<Node<Message>>, I: IntoIterator<Item = (u64, N)>>(
        mut self,
        i: I,
    ) -> Self {
        self.children
            .0
            .extend(i.into_iter().map(|(key, value)| (key, value.into())));
        self
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(Message) -> NewMessage + 'static,
    ) -> KeyedElement<NewMessage> {
        self.do_map(Rc::new(f))
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> KeyedElement<NewMessage> {
        let Element {
            name,
            ns,
            attrs,
            listeners,
            children,
            node,
        } = self;
        let listeners = listeners
            .into_iter()
            .map(|listener| listener.do_map(f.clone()))
            .collect();
        let children = Keyed(
            children
                .0
                .into_iter()
                .map(|(k, v)| (k, v.do_map(f.clone())))
                .collect(),
        );
        Element {
            name,
            ns,
            attrs,
            listeners,
            children,
            node,
        }
    }
}

pub trait Children {
    type Message;
    fn new() -> Self;
    fn create(&mut self, node: &web::Node, mailbox: &Mailbox<Self::Message>);
    fn patch(&mut self, old: &mut Self, old_node: &web::Node, mailbox: &Mailbox<Self::Message>);
}

impl<Message: 'static> Children for NonKeyed<Message> {
    type Message = Message;

    fn new() -> Self {
        NonKeyed(Vec::new())
    }

    fn create(&mut self, node: &web::Node, mailbox: &Mailbox<Message>) {
        for child in &mut self.0 {
            let child_node = child.create(mailbox);
            node.append_child(&child_node).expect("append_child");
        }
    }

    fn patch(&mut self, old: &mut Self, old_node: &web::Node, mailbox: &Mailbox<Message>) {
        for (old, new) in old.0.iter_mut().zip(&mut self.0) {
            new.patch(old, mailbox);
        }

        for old in old.0.iter().skip(self.0.len()) {
            let old_node = old.node().expect("old.node");
            let parent_node = old_node.parent_node().expect("old.parent_node");
            parent_node.remove_child(&old_node).expect("remove_child");
        }

        for new in self.0.iter_mut().skip(old.0.len()) {
            let new_node = new.create(mailbox);
            old_node
                .append_child(&new_node)
                .expect("old_node.append_child");
        }
    }
}

impl<Message: 'static> Children for Keyed<Message> {
    type Message = Message;

    fn new() -> Self {
        Keyed(Vec::new())
    }

    fn create(&mut self, node: &web::Node, mailbox: &Mailbox<Message>) {
        for (_, child) in &mut self.0 {
            let child_node = child.create(mailbox);
            node.append_child(&child_node).expect("append_child");
        }
    }

    fn patch(&mut self, old: &mut Self, parent_node: &web::Node, mailbox: &Mailbox<Message>) {
        if self.0.is_empty() {
            parent_node.set_text_content(Some(""));
            return;
        }

        let mut skip: usize = 0;
        for ((new_key, new_node), (old_key, ref mut old_node)) in self.0.iter_mut().zip(&mut old.0)
        {
            if new_key == old_key {
                new_node.patch(old_node, mailbox);
                skip += 1;
            } else {
                break;
            }
        }
        let new = &mut self.0[skip..];
        let old = &mut old.0[skip..];

        let mut skip_end = 0;
        for ((new_key, new_node), (old_key, ref mut old_node)) in
            new.iter_mut().rev().zip(old.iter_mut().rev())
        {
            if new_key == old_key {
                new_node.patch(old_node, mailbox);
                skip_end += 1;
            } else {
                break;
            }
        }
        let new_len = new.len();
        let old_len = old.len();
        let new = &mut new[..new_len - skip_end];
        let old = &mut old[..old_len - skip_end];

        if new.is_empty() && old.is_empty() {
            return;
        }
        let mut key_to_old_index = HashMap::default();
        for (index, (key, _)) in (skip..).zip(old.iter_mut()) {
            key_to_old_index.insert(key.clone(), index);
        }

        let child_nodes = parent_node.child_nodes();
        let child_nodes_length = child_nodes.length();
        for (index, (key, new_node)) in (skip..).zip(new.iter_mut()) {
            let reordered = if let Some(old_index) = key_to_old_index.remove(key) {
                let (_, ref mut old_node) = old[old_index - skip];
                new_node.patch(old_node, mailbox);
                old_index != index
            } else {
                new_node.create(mailbox);
                true
            };
            if reordered {
                if index as u32 > child_nodes_length {
                    parent_node.append_child(&new_node.node().unwrap()).unwrap();
                } else {
                    let next_sibling = child_nodes.get(index as u32 + 1);
                    parent_node
                        .insert_before(&new_node.node().unwrap(), next_sibling.as_ref())
                        .unwrap();
                }
            }
        }

        for index in key_to_old_index.values() {
            old[*index - skip].1.remove();
        }
    }
}
