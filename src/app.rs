use crate::{Mailbox, Node, Text};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys as web;

pub trait App: Sized + 'static {
    type Message;

    fn update(&mut self, _mailbox: &Mailbox<Self::Message>, _message: Self::Message) {}
    fn render(&self) -> Node<Self::Message>;
}

pub struct Instance<A: App> {
    inner: Rc<Inner<A>>,
}

struct Inner<A: App> {
    app: RefCell<A>,
    node: RefCell<web::Node>,
    vnode: RefCell<Node<A::Message>>,
    queue: RefCell<Vec<A::Message>>,
    is_updating: RefCell<bool>,
}

impl<A: App> Instance<A> {
    fn send(&self, message: A::Message) {
        if *self.inner.is_updating.borrow() {
            self.inner.queue.borrow_mut().push(message);
            return;
        }
        self.inner.is_updating.replace(true);
        let mailbox = self.mailbox();
        self.inner.app.borrow_mut().update(&mailbox, message);
        while !self.inner.queue.borrow().is_empty() {
            let message = self.inner.queue.borrow_mut().remove(0);
            self.inner.app.borrow_mut().update(&mailbox, message);
        }
        self.inner.is_updating.replace(false);
        self.render();
    }

    fn render(&self) {
        let mut new_vnode = self.inner.app.borrow().render();
        let new_node = new_vnode.patch(&mut self.inner.vnode.borrow_mut(), self.mailbox());
        self.inner.vnode.replace(new_vnode);
        self.inner.node.replace(new_node);
    }

    fn mailbox(&self) -> Mailbox<A::Message> {
        let cloned = self.clone();
        Mailbox::new(move |message| {
            cloned.send(message);
        })
    }
}

impl<A: App> std::clone::Clone for Instance<A> {
    fn clone(&self) -> Self {
        Instance {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<A: App> std::fmt::Debug for Instance<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Instance").finish()
    }
}

pub fn start<A: App>(app: A, node: web::Node) -> Mailbox<A::Message> {
    let mut vnode = Text::new("!");
    let new_node = vnode.create().into();
    node.parent_node()
        .unwrap()
        .replace_child(&new_node, &node)
        .unwrap();
    let instance = Instance {
        inner: Rc::new(Inner {
            app: RefCell::new(app),
            node: RefCell::new(new_node),
            vnode: RefCell::new(vnode.into()),
            is_updating: RefCell::new(false),
            queue: RefCell::new(Vec::new()),
        }),
    };
    instance.render();
    instance.mailbox()
}
