use crate::{Subscription, Unsubscribe};
use std::rc::Rc;

pub struct Mailbox<Message: 'static> {
    func: Rc<Fn(Message)>,
}

impl<Message: 'static> Mailbox<Message> {
    pub fn new(func: impl Fn(Message) + 'static) -> Self {
        Mailbox {
            func: Rc::new(func),
        }
    }

    pub fn send(&self, message: Message) {
        (self.func)(message)
    }

    pub fn subscribe<S: Subscription + 'static>(
        &self,
        subscription: S,
        f: impl Fn(S::Message) -> Message + 'static,
    ) -> Unsubscribe {
        let cloned = self.clone();
        subscription.subscribe(Rc::new(move |message| cloned.send(f(message))))
    }

    pub fn map<NewMessage: 'static>(
        self,
        f: impl Fn(NewMessage) -> Message + 'static,
    ) -> Mailbox<NewMessage> {
        Mailbox {
            func: Rc::new(move |message| (self.func)(f(message))),
        }
    }
}

impl<Message> Clone for Mailbox<Message> {
    fn clone(&self) -> Self {
        Mailbox {
            func: self.func.clone(),
        }
    }
}

impl<Message> std::fmt::Debug for Mailbox<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Mailbox").finish()
    }
}
