use crate::Node;
use std::cell::Cell;

pub struct Observe<T> {
    changed: Cell<bool>,
    t: T,
}

impl<T> Observe<T> {
    pub fn new(t: T) -> Self {
        Observe {
            changed: Cell::new(true),
            t,
        }
    }

    pub fn render<Message>(&self, f: impl Fn(&T) -> Node<Message>) -> Node<Message> {
        if self.changed.get() {
            self.changed.replace(false);
            f(self)
        } else {
            Node::Keep
        }
    }
}

impl<T> std::ops::Deref for Observe<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> std::ops::DerefMut for Observe<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.changed.replace(true);
        &mut self.t
    }
}
