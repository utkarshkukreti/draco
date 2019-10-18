use crate::{Mailbox, Node, Text};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

pub trait App: Sized + 'static {
    type Message;

    fn update(&mut self, _message: Self::Message, _mailbox: &Mailbox<Self::Message>) {}
    fn view(&self) -> Node<Self::Message>;
}

struct Instance<A: App> {
    inner: Rc<Inner<A>>,
}

struct Inner<A: App> {
    app: RefCell<A>,
    node: RefCell<web::Node>,
    vnode: RefCell<Node<A::Message>>,
    queue: RefCell<Vec<A::Message>>,
    is_updating: Cell<bool>,
}

impl<A: App> Instance<A> {
    fn send(&self, message: A::Message) {
        self.push(message);
        self.update();
    }

    fn push(&self, message: A::Message) {
        self.inner.queue.borrow_mut().push(message);
    }

    fn update(&self) {
        // If we were called from inside the `while` loop below, bail out; the message will be
        // processed by the loop later.
        if self.inner.is_updating.get() {
            return;
        }

        self.inner.is_updating.replace(true);

        let mailbox = self.mailbox();

        while !self.inner.queue.borrow().is_empty() {
            let message = self.inner.queue.borrow_mut().remove(0);
            self.inner.app.borrow_mut().update(message, &mailbox);
        }

        self.inner.is_updating.replace(false);

        self.render();
    }

    fn render(&self) {
        let mut new_vnode = self.inner.app.borrow().view();
        let new_node = new_vnode.patch(&mut self.inner.vnode.borrow_mut(), &self.mailbox());
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
        .unwrap_throw()
        .replace_child(&new_node, &node)
        .unwrap_throw();
    let instance = Instance {
        inner: Rc::new(Inner {
            app: RefCell::new(app),
            node: RefCell::new(new_node),
            vnode: RefCell::new(vnode.into()),
            is_updating: Cell::new(false),
            queue: RefCell::new(Vec::new()),
        }),
    };
    instance.render();
    instance.mailbox()
}
