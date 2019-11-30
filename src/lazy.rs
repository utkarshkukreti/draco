use crate::{Mailbox, VNode};
use derivative::Derivative;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct Lazy<Message: 'static> {
    hash: u64,
    vnode: Option<Box<VNode<Message>>>,
    node: Option<web::Node>,
    #[derivative(Debug = "ignore")]
    view: Box<dyn Fn() -> VNode<Message>>,
}

impl<Message: 'static> Lazy<Message> {
    pub fn new<T: Hash + 'static>(t: T, view: fn(&T) -> VNode<Message>) -> Self {
        let mut hasher = fxhash::FxHasher::default();
        t.hash(&mut hasher);
        (view as usize).hash(&mut hasher);
        let hash = hasher.finish();
        Lazy {
            hash,
            vnode: None,
            node: None,
            view: Box::new(move || view(&t)),
        }
    }

    pub fn new_with<T: Hash + 'static, Arg: 'static>(
        t: T,
        arg: Arg,
        view: fn(&T, &Arg) -> VNode<Message>,
    ) -> Self {
        let mut hasher = fxhash::FxHasher::default();
        t.hash(&mut hasher);
        (view as usize).hash(&mut hasher);
        let hash = hasher.finish();
        Lazy {
            hash,
            vnode: None,
            node: None,
            view: Box::new(move || view(&t, &arg)),
        }
    }

    pub fn create(&mut self, mailbox: &Mailbox<Message>) -> web::Node {
        let mut vnode = (self.view)();
        let node = vnode.create(mailbox);
        self.node = Some(node.clone());
        self.vnode = Some(Box::new(vnode));
        node
    }

    pub fn patch(&mut self, old: &mut Self, mailbox: &Mailbox<Message>) -> web::Node {
        let mut old_vnode = *old.vnode.take().unwrap_throw();
        let old_node = old_vnode.node().unwrap_throw();
        if self.hash == old.hash {
            self.vnode = Some(Box::new(old_vnode));
            self.node = Some(old_node.clone());
            return old_node;
        }
        let mut vnode = (self.view)();
        let node = vnode.patch(&mut old_vnode, mailbox);
        self.node = Some(node.clone());
        self.vnode = Some(Box::new(vnode));
        node
    }

    pub fn did_create(&self, node: &web::Node, mailbox: &Mailbox<Message>) {
        if let Some(ref vnode) = self.vnode {
            vnode.did_create(node, mailbox);
        };
    }

    pub fn did_remove(&self, mailbox: &Mailbox<Message>) {
        if let Some(ref vnode) = self.vnode {
            vnode.did_remove(mailbox);
        };
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> Lazy<NewMessage> {
        Lazy::new_with(self.hash, (self.view, f), |_, (view, f)| {
            view().do_map(f.clone())
        })
    }

    pub fn node(&self) -> Option<web::Node> {
        self.node.clone()
    }
}
