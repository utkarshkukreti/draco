use crate::{Subscription, Unsubscribe};
use std::any::Any;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

pub struct Mailbox<Message: 'static> {
    inner: Rc<Inner<Message>>,
}

struct Inner<Message: 'static> {
    f: Box<dyn Fn(Message)>,
    // XXX: Is this a good idea?
    stash: RefCell<Vec<Box<dyn Any>>>,
}

impl<Message: 'static> Mailbox<Message> {
    pub fn new(f: impl Fn(Message) + 'static) -> Self {
        Mailbox {
            inner: Rc::new(Inner {
                f: Box::new(f),
                stash: RefCell::new(Vec::new()),
            }),
        }
    }

    pub fn send(&self, message: Message) {
        (self.inner.f)(message)
    }

    pub fn send_after(&self, timeout: i32, f: impl Fn() -> Message + 'static) {
        let cloned = self.clone();
        let closure = Closure::wrap(Box::new(move || {
            cloned.send(f());
        }) as Box<dyn FnMut()>);
        web::window()
            .unwrap_throw()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                timeout,
            )
            .unwrap_throw();
        // TODO: Drop the closure when it is first called.
        self.stash(closure);
    }

    pub fn subscribe<S: Subscription + 'static>(
        &self,
        subscription: S,
        f: impl Fn(S::Message) -> Message + 'static,
    ) -> Unsubscribe {
        let cloned = self.clone();
        subscription.subscribe(Rc::new(move |message| cloned.send(f(message))))
    }

    pub fn subscribe_forever<S: Subscription + 'static>(
        &self,
        subscription: S,
        f: impl Fn(S::Message) -> Message + 'static,
    ) {
        let cloned = self.clone();
        let unsubscribe = subscription.subscribe(Rc::new(move |message| cloned.send(f(message))));
        self.stash(unsubscribe);
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(NewMessage) -> Message + 'static,
    ) -> Mailbox<NewMessage> {
        Mailbox {
            inner: Rc::new(Inner {
                f: Box::new(move |message| (self.inner.f)(f(message))),
                stash: RefCell::new(Vec::new()),
            }),
        }
    }

    pub fn spawn<F>(&self, future: F, f: impl Fn(F::Output) -> Message + 'static)
    where
        F: Future + 'static,
    {
        let cloned = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            cloned.send(f(future.await));
        });
    }

    fn stash(&self, t: impl Any) {
        self.inner.stash.borrow_mut().push(Box::new(t));
    }
}

impl<Message> Clone for Mailbox<Message> {
    fn clone(&self) -> Self {
        Mailbox {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<Message> std::fmt::Debug for Mailbox<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Mailbox").finish()
    }
}
