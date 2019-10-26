use crate::{aspect, property, Aspect, Attribute, Listener, Mailbox, Property, VNode, S};
// use std::collections::HashMap;
use fxhash::FxHashMap as HashMap;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

pub type VNonKeyedElement<Message> = VElement<NonKeyed<Message>>;
pub type VKeyedElement<Message> = VElement<Keyed<Message>>;

#[derive(Debug)]
pub struct VElement<C: Children> {
    pub(crate) name: &'static str,
    ns: Ns,
    class: S,
    aspects: Vec<Aspect<C::Message>>,
    children: C,
    node: Option<web::Element>,
}

#[derive(Debug)]
pub enum Ns {
    Html,
    Svg,
}

#[derive(Debug, Default)]
pub struct Keyed<Message: 'static>(Vec<(u64, VNode<Message>)>);

#[derive(Debug, Default)]
pub struct NonKeyed<Message: 'static>(Vec<VNode<Message>>);

pub fn h<Message: 'static>(name: &'static str) -> VNonKeyedElement<Message> {
    VElement::new(Ns::Html, name)
}

pub fn s<Message: 'static>(name: &'static str) -> VNonKeyedElement<Message> {
    VElement::new(Ns::Svg, name)
}

impl<C: Children> VElement<C>
where
    C::Message: 'static,
{
    pub fn new(ns: Ns, name: &'static str) -> Self {
        VElement {
            name,
            ns,
            aspects: Vec::new(),
            class: "".into(),
            children: C::new(),
            node: None,
        }
    }

    pub fn class(mut self, value: impl Into<S>) -> Self {
        self.class = value.into();
        self
    }

    pub fn attribute(mut self, name: impl Into<S>, value: impl Into<S>) -> Self {
        self.aspects.push(Attribute::new(name, value).into());
        self
    }

    pub fn property(mut self, name: impl Into<S>, value: impl Into<property::Value>) -> Self {
        self.aspects.push(Property::new(name, value).into());
        self
    }

    pub fn on<N: Into<S>>(
        mut self,
        name: N,
        mut handler: impl FnMut(web::Event) -> C::Message + 'static,
    ) -> Self {
        self.aspects
            .push(Listener::new(name, move |event| Some(handler(event))).into());
        self
    }

    pub fn on_<N: Into<S>>(
        mut self,
        name: N,
        handler: impl FnMut(web::Event) -> Option<C::Message> + 'static,
    ) -> Self {
        self.aspects.push(Listener::new(name, handler).into());
        self
    }

    pub fn on_input(self, mut handler: impl FnMut(String) -> C::Message + 'static) -> Self {
        self.on_("input", move |event| {
            Some(handler(
                js_sys::Reflect::get(&&event.target()?, &JsValue::from_str("value"))
                    .ok()?
                    .as_string()?,
            ))
        })
    }

    pub fn on_checked(self, mut handler: impl FnMut(bool) -> C::Message + 'static) -> Self {
        self.on_("input", move |event| {
            Some(handler(
                js_sys::Reflect::get(&&event.target()?, &JsValue::from_str("checked"))
                    .ok()?
                    .as_bool()?,
            ))
        })
    }

    pub fn create(&mut self, mailbox: &Mailbox<C::Message>) -> web::Element {
        let document = web::window().unwrap_throw().document().unwrap_throw();

        let element = match self.ns {
            Ns::Html => document
                .create_element(wasm_bindgen::intern(&self.name))
                .unwrap_throw(),
            Ns::Svg => document
                .create_element_ns(
                    Some("http://www.w3.org/2000/svg"),
                    wasm_bindgen::intern(&self.name),
                )
                .unwrap_throw(),
        };

        self.children
            .create(element.as_ref() as &web::Node, mailbox);

        aspect::patch(&mut self.aspects, &[], &element, mailbox);

        if !self.class.is_empty() {
            element.set_class_name(wasm_bindgen::intern(&self.class));
        }

        self.node = Some(element.clone());

        element
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<C::Message>) -> web::Element {
        debug_assert!(self.name == old.name);
        let old_element = old.node.take().unwrap_throw();

        self.children
            .patch(&mut old.children, old_element.as_ref(), mailbox);

        aspect::patch(&mut self.aspects, &old.aspects, &old_element, mailbox);

        if self.class != old.class {
            old_element.set_class_name(wasm_bindgen::intern(&self.class));
        }

        self.node = Some(old_element.clone());

        old_element
    }

    pub fn node(&self) -> Option<web::Element> {
        self.node.clone()
    }
}

impl<Message: 'static> VNonKeyedElement<Message> {
    pub fn push<N: Into<VNode<Message>>>(mut self, vnode: N) -> Self {
        self.children.0.push(vnode.into());
        self
    }

    pub fn append<N: Into<VNode<Message>>, I: IntoIterator<Item = N>>(mut self, i: I) -> Self {
        self.children.0.extend(i.into_iter().map(Into::into));
        self
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(Message) -> NewMessage + 'static,
    ) -> VNonKeyedElement<NewMessage> {
        self.do_map(Rc::new(f))
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> VNonKeyedElement<NewMessage> {
        let VElement {
            name,
            ns,
            class,
            aspects,
            children,
            node,
        } = self;
        let aspects = aspects
            .into_iter()
            .map(|aspect| aspect.do_map(f.clone()))
            .collect();
        let children = NonKeyed(
            children
                .0
                .into_iter()
                .map(|n| n.do_map(f.clone()))
                .collect(),
        );
        VElement {
            name,
            ns,
            class,
            aspects,
            children,
            node,
        }
    }
}

impl<Message: 'static> VKeyedElement<Message> {
    pub fn push<N: Into<VNode<Message>>>(mut self, key: u64, vnode: N) -> Self {
        self.children.0.push((key, vnode.into()));
        self
    }

    pub fn append<N: Into<VNode<Message>>, I: IntoIterator<Item = (u64, N)>>(
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
    ) -> VKeyedElement<NewMessage> {
        self.do_map(Rc::new(f))
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> VKeyedElement<NewMessage> {
        let VElement {
            name,
            ns,
            class,
            aspects,
            children,
            node,
        } = self;
        let aspects = aspects
            .into_iter()
            .map(|aspect| aspect.do_map(f.clone()))
            .collect();
        let children = Keyed(
            children
                .0
                .into_iter()
                .map(|(k, v)| (k, v.do_map(f.clone())))
                .collect(),
        );
        VElement {
            name,
            ns,
            class,
            aspects,
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
            node.append_child(&child_node).unwrap_throw();
        }
    }

    fn patch(&mut self, old: &mut Self, old_node: &web::Node, mailbox: &Mailbox<Message>) {
        for (old, new) in old.0.iter_mut().zip(&mut self.0) {
            new.patch(old, mailbox);
        }

        for old in old.0.iter().skip(self.0.len()) {
            let old_node = old.node().unwrap_throw();
            let parent_node = old_node.parent_node().unwrap_throw();
            parent_node.remove_child(&old_node).unwrap_throw();
        }

        for new in self.0.iter_mut().skip(old.0.len()) {
            let new_node = new.create(mailbox);
            old_node.append_child(&new_node).unwrap_throw();
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
            node.append_child(&child_node).unwrap_throw();
        }
    }

    fn patch(&mut self, old: &mut Self, parent_node: &web::Node, mailbox: &Mailbox<Message>) {
        if self.0.is_empty() {
            parent_node.set_text_content(Some(""));
            return;
        }

        let mut skip: usize = 0;
        for ((new_key, new_vnode), (old_key, ref mut old_vnode)) in
            self.0.iter_mut().zip(&mut old.0)
        {
            if new_key == old_key {
                new_vnode.patch(old_vnode, mailbox);
                skip += 1;
            } else {
                break;
            }
        }
        let new = &mut self.0[skip..];
        let old = &mut old.0[skip..];

        let mut skip_end = 0;
        for ((new_key, new_vnode), (old_key, ref mut old_vnode)) in
            new.iter_mut().rev().zip(old.iter_mut().rev())
        {
            if new_key == old_key {
                new_vnode.patch(old_vnode, mailbox);
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
        for (index, (key, new_vnode)) in (skip..).zip(new.iter_mut()) {
            let reordered = if let Some(old_index) = key_to_old_index.remove(key) {
                let (_, ref mut old_vnode) = old[old_index - skip];
                new_vnode.patch(old_vnode, mailbox);
                old_index != index
            } else {
                new_vnode.create(mailbox);
                true
            };
            if reordered {
                let next_sibling = child_nodes.get(index as u32 + 1);
                parent_node
                    .insert_before(&new_vnode.node().unwrap_throw(), next_sibling.as_ref())
                    .unwrap_throw();
            }
        }

        for index in key_to_old_index.values() {
            old[*index - skip].1.remove();
        }
    }
}
