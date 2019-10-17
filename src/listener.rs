use crate::{Mailbox, S};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

pub struct Listener<Message> {
    name: S,
    handler: Option<Box<dyn FnMut(web::Event) -> Message>>,
    closure: Option<Closure<dyn FnMut(web::Event)>>,
}

impl<Message: 'static> Listener<Message> {
    pub fn new(name: impl Into<S>, handler: impl FnMut(web::Event) -> Message + 'static) -> Self {
        Listener {
            name: name.into(),
            handler: Some(Box::new(handler)),
            closure: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> Listener<NewMessage> {
        let Listener {
            name,
            mut handler,
            closure,
        } = self;
        let handler = match handler.take() {
            Some(mut handler) => Some(Box::new(move |event| f(handler(event)))
                as Box<dyn FnMut(web::Event) -> NewMessage>),
            None => None,
        };
        Listener {
            name,
            handler,
            closure,
        }
    }
}

impl<Message> std::fmt::Debug for Listener<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Listener")
            .field("name", &self.name)
            .finish()
    }
}

impl<Message: 'static> Listener<Message> {
    pub fn attach(&mut self, element: &web::Element, mailbox: &Mailbox<Message>) {
        let mailbox = mailbox.clone();
        let mut handler = self.handler.take().unwrap_throw();
        let closure = Closure::wrap(
            Box::new(move |event: web::Event| mailbox.send(handler(event)))
                as Box<dyn FnMut(web::Event) + 'static>,
        );
        (element.as_ref() as &web::EventTarget)
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .unwrap_throw();
        self.closure = Some(closure);
    }

    pub fn detach(&self, element: &web::Element) {
        let closure = self.closure.as_ref().unwrap_throw();
        (element.as_ref() as &web::EventTarget)
            .remove_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .unwrap_throw();
    }
}
