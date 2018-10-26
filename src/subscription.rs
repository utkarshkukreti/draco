use js_sys as js;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys as web;

pub type Send<Message> = Rc<Fn(Message)>;

pub trait Subscription {
    type Message;

    fn subscribe(self, send: Send<Self::Message>) -> Unsubscribe;
}

pub struct Unsubscribe(Option<Box<FnMut()>>);

impl Unsubscribe {
    pub fn new(f: impl FnMut() + 'static) -> Self {
        Unsubscribe(Some(Box::new(f)))
    }
}

impl std::fmt::Debug for Unsubscribe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Unsubscribe").finish()
    }
}

impl Drop for Unsubscribe {
    fn drop(&mut self) {
        if let Some(mut f) = self.0.take() {
            f()
        }
    }
}

#[derive(Debug)]
pub struct OnWindow {
    name: String,
}

impl OnWindow {
    pub fn new<N: Into<String>>(name: N) -> Self {
        OnWindow { name: name.into() }
    }
}

impl Subscription for OnWindow {
    type Message = web::Event;

    fn subscribe(self, send: Send<Self::Message>) -> Unsubscribe {
        let window = web::window().unwrap();
        let closure = Closure::wrap(Box::new(move |event: web::Event| {
            send(event);
        }) as Box<FnMut(web::Event)>);
        (window.as_ref() as &web::EventTarget)
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .unwrap();
        Unsubscribe::new(move || {
            (window.as_ref() as &web::EventTarget)
                .remove_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
                .unwrap();
        })
    }
}

#[derive(Debug)]
pub struct Interval {
    ms: i32,
}

impl Interval {
    pub fn new(ms: i32) -> Self {
        Interval { ms }
    }
}

impl Subscription for Interval {
    type Message = ();

    fn subscribe(self, send: Send<Self::Message>) -> Unsubscribe {
        let window = web::window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            send(());
        }) as Box<FnMut()>);
        let id = window
            .set_interval_with_callback_and_timeout_and_arguments(
                closure.as_ref().unchecked_ref(),
                self.ms,
                &js::Array::new(),
            )
            .unwrap();
        Unsubscribe::new(move || {
            // We need to move `closure` here so that it isn't dropped too early.
            let _ = closure;
            window.clear_interval_with_handle(id);
        })
    }
}
