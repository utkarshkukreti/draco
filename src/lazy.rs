use crate::{Mailbox, Node};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

pub struct Lazy<Message: 'static> {
    hash: u64,
    node: Option<Box<Node<Message>>>,
    view: Box<dyn Fn() -> Node<Message>>,
}

impl<Message: 'static + std::fmt::Debug> std::fmt::Debug for Lazy<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("hash", &self.hash)
            .field("node", &self.node)
            .finish()
    }
}

impl<Message: 'static> Lazy<Message> {
    pub fn new<T: Hash + 'static>(t: T, view: fn(&T) -> Node<Message>) -> Self {
        let mut hasher = fxhash::FxHasher::default();
        t.hash(&mut hasher);
        (view as usize).hash(&mut hasher);
        let hash = hasher.finish();
        Lazy {
            hash,
            node: None,
            view: Box::new(move || view(&t)),
        }
    }

    pub fn new_with<T: Hash + 'static, Arg: 'static>(
        t: T,
        arg: Arg,
        view: fn(&T, &Arg) -> Node<Message>,
    ) -> Self {
        let mut hasher = fxhash::FxHasher::default();
        t.hash(&mut hasher);
        (view as usize).hash(&mut hasher);
        let hash = hasher.finish();
        Lazy {
            hash,
            node: None,
            view: Box::new(move || view(&t, &arg)),
        }
    }

    pub fn create(&mut self, mailbox: &Mailbox<Message>) -> web::Node {
        let mut node = (self.view)();
        let web_node = node.create(mailbox);
        self.node = Some(Box::new(node));
        web_node
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<Message>) -> web::Node {
        let mut old_node = *old.node.take().unwrap_throw();
        let old_web_node = old_node.node().unwrap_throw();
        if self.hash == old.hash {
            self.node = Some(Box::new(old_node));
            return old_web_node;
        }
        let mut node = (self.view)();
        let web_node = node.patch(&mut old_node, mailbox);
        self.node = Some(Box::new(node));
        web_node
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> Lazy<NewMessage> {
        Lazy::new_with(self.hash, (self.view, f), move |_, (view, f)| {
            view().do_map(f.clone())
        })
    }

    pub fn node(&self) -> Option<web::Node> {
        self.node.as_ref().and_then(|node| node.node())
    }
}
