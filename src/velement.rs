use crate::{aspect, property, Aspect, Attribute, Listener, Mailbox, Property, VNode, S};
// use std::collections::HashMap;
use derivative::Derivative;
use fxhash::FxHashMap as HashMap;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::{JsCast, JsValue};
use web_sys as web;

pub type VNonKeyedElement<Message> = VElement<NonKeyed<Message>>;
pub type VKeyedElement<Message> = VElement<Keyed<Message>>;

#[derive(Derivative)]
#[derivative(Debug(bound = "C: std::fmt::Debug"))]
pub struct VElement<C: Children> {
    pub(crate) name: &'static str,
    ns: Ns,
    class: S,
    aspects: Vec<Aspect<C::Message>>,
    children: C,
    #[derivative(Debug = "ignore")]
    ref_: Option<Box<dyn Fn(Option<web::Element>) -> C::Message>>,
    node: Option<web::Element>,
}

#[derive(Debug)]
pub enum Ns {
    Html,
    Svg,
}

#[derive(Default, Derivative)]
#[derivative(Debug(bound = ""))]
pub struct Keyed<Message: 'static>(Vec<(u64, VNode<Message>)>);

#[derive(Default, Derivative)]
#[derivative(Debug(bound = ""))]
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
            ref_: None,
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
        handler: impl Fn(web::Event) -> C::Message + 'static,
    ) -> Self {
        self.aspects
            .push(Listener::new(name, move |event| Some(handler(event))).into());
        self
    }

    pub fn on_<N: Into<S>>(
        mut self,
        name: N,
        handler: impl Fn(web::Event) -> Option<C::Message> + 'static,
    ) -> Self {
        self.aspects.push(Listener::new(name, handler).into());
        self
    }

    pub fn on_input(self, handler: impl Fn(String) -> C::Message + 'static) -> Self {
        self.on_("input", move |event| {
            Some(handler(
                js_sys::Reflect::get(&&event.target()?, &JsValue::from_str("value"))
                    .ok()?
                    .as_string()?,
            ))
        })
    }

    pub fn on_checked(self, handler: impl Fn(bool) -> C::Message + 'static) -> Self {
        self.on_("input", move |event| {
            Some(handler(
                js_sys::Reflect::get(&&event.target()?, &JsValue::from_str("checked"))
                    .ok()?
                    .as_bool()?,
            ))
        })
    }

    pub fn ref_(mut self, handler: impl Fn(Option<web::Element>) -> C::Message + 'static) -> Self {
        self.ref_ = Some(Box::new(handler));
        self
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

        self.did_create(element.as_ref(), mailbox);

        element
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<C::Message>) -> web::Element {
        debug_assert!(self.name == old.name);
        let old_element = old.node.clone().unwrap_throw();

        self.children
            .patch(&mut old.children, old_element.as_ref(), mailbox);

        aspect::patch(&mut self.aspects, &old.aspects, &old_element, mailbox);

        if self.class != old.class {
            old_element.set_class_name(wasm_bindgen::intern(&self.class));
        }

        self.node = Some(old_element.clone());

        old_element
    }

    pub fn did_create(&self, node: &web::Node, mailbox: &Mailbox<C::Message>) {
        if let Some(ref ref_) = self.ref_ {
            mailbox.send(ref_(Some(
                node.dyn_ref::<web::Element>().unwrap_throw().clone(),
            )));
        }
    }

    pub fn did_remove(&self, mailbox: &Mailbox<C::Message>) {
        if let Some(ref ref_) = self.ref_ {
            mailbox.send(ref_(None));
        }
    }

    pub fn node(&self) -> Option<web::Element> {
        self.node.clone()
    }

    pub fn with<W: With<C>>(mut self, with: W) -> Self {
        with.with(&mut self.children);
        self
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
            ref_,
            node,
        } = self;
        let aspects = aspects
            .into_iter()
            .map(|aspect| aspect.do_map(f.clone()))
            .collect();
        let ref_ = {
            let f = f.clone();
            ref_.map(|ref_| {
                Box::new(move |el| f(ref_(el))) as Box<dyn Fn(Option<web::Element>) -> NewMessage>
            })
        };
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
            ref_,
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
            ref_,
            node,
        } = self;
        let aspects = aspects
            .into_iter()
            .map(|aspect| aspect.do_map(f.clone()))
            .collect();
        let ref_ = {
            let f = f.clone();
            ref_.map(|ref_| {
                Box::new(move |el| f(ref_(el))) as Box<dyn Fn(Option<web::Element>) -> NewMessage>
            })
        };
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
            ref_,
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
            old.remove(mailbox);
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
        let new = &mut self.0;
        let old = &mut old.0;

        if new.is_empty() {
            parent_node.set_text_content(Some(""));
            return;
        }

        let mut start_index: usize = 0;
        for ((new_key, new_vnode), (old_key, ref mut old_vnode)) in
            new.iter_mut().zip(old.iter_mut())
        {
            if new_key == old_key {
                new_vnode.patch(old_vnode, mailbox);
                start_index += 1;
            } else {
                break;
            }
        }

        let mut skip_end = 0;
        for ((new_key, new_vnode), (old_key, ref mut old_vnode)) in new[start_index..]
            .iter_mut()
            .rev()
            .zip(old[start_index..].iter_mut().rev())
        {
            if new_key == old_key {
                new_vnode.patch(old_vnode, mailbox);
                skip_end += 1;
            } else {
                break;
            }
        }

        let end_index_new = new.len() - skip_end;
        let end_index_old = old.len() - skip_end;

        if start_index == end_index_new && start_index == end_index_old {
            return;
        }

        let mut key_to_old_index = HashMap::default();
        for (index, (key, _)) in (start_index..).zip(old[start_index..end_index_old].iter_mut()) {
            key_to_old_index.insert(key.clone(), index);
        }

        for (index, (key, new_vnode)) in
            (start_index..).zip(new[start_index..end_index_new].iter_mut())
        {
            let reordered = if let Some(old_index) = key_to_old_index.remove(key) {
                let (_, ref mut old_vnode) = old[old_index];
                new_vnode.patch(old_vnode, mailbox);
                old_index != index
            } else {
                new_vnode.create(mailbox);
                true
            };
            if reordered {
                let next_sibling = old.get(index + 1).and_then(|(_, vnode)| vnode.node());
                parent_node
                    .insert_before(&new_vnode.node().unwrap_throw(), next_sibling.as_ref())
                    .unwrap_throw();
            }
        }

        for index in key_to_old_index.values() {
            old[*index].1.remove(mailbox);
        }
    }
}

pub trait With<C: Children> {
    fn with(self, element: &mut C);
}

macro_rules! go {
    () => {};
    ($first:ident $($rest:ident)*) => {
        impl<C: Children, $first: With<C> $(,$rest: With<C>)*> With<C> for ($first, $($rest,)*) {
            fn with(self, children: &mut C) {
                #[allow(non_snake_case)]
                let ($first, $($rest,)*) = self;
                $first.with(children);
                $({ $rest.with(children); })*
            }
        }

        go! { $($rest)* }
    }
}

go! {
    T1 T2 T3 T4 T5 T6 T7 T8 T9 T10
    T11 T12 T13 T14 T15 T16 T17 T18 T19 T20
    T21 T22 T23 T24
}

impl<Message: 'static, T: Into<VNode<Message>>> With<NonKeyed<Message>> for T {
    fn with(self, children: &mut NonKeyed<Message>) {
        children.0.push(self.into());
    }
}

impl<Message: 'static, T: Into<VNode<Message>>> With<Keyed<Message>> for (u64, T) {
    fn with(self, children: &mut Keyed<Message>) {
        children.0.push((self.0, self.1.into()));
    }
}
