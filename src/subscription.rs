use derivative::Derivative;
use js_sys as js;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

pub type Send<Message> = Rc<dyn Fn(Message)>;

pub trait Subscription {
    type Message;

    fn subscribe(self, send: Send<Self::Message>) -> Unsubscribe;
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Unsubscribe(#[derivative(Debug = "ignore")] Option<Box<dyn FnMut()>>);

impl Unsubscribe {
    pub fn new(f: impl FnMut() + 'static) -> Self {
        Unsubscribe(Some(Box::new(f)))
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
        let window = web::window().unwrap_throw();
        let closure = Closure::wrap(Box::new(move |event: web::Event| {
            send(event);
        }) as Box<dyn FnMut(web::Event)>);
        (window.as_ref() as &web::EventTarget)
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .unwrap_throw();
        Unsubscribe::new(move || {
            (window.as_ref() as &web::EventTarget)
                .remove_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
                .unwrap_throw();
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
        let window = web::window().unwrap_throw();
        let closure = Closure::wrap(Box::new(move || {
            send(());
        }) as Box<dyn FnMut()>);
        let id = window
            .set_interval_with_callback_and_timeout_and_arguments(
                closure.as_ref().unchecked_ref(),
                self.ms,
                &js::Array::new(),
            )
            .unwrap_throw();
        Unsubscribe::new(move || {
            // We need to move `closure` here so that it isn't dropped too early.
            let _ = closure;
            window.clear_interval_with_handle(id);
        })
    }
}

#[derive(Debug)]
pub struct AnimationFrame {}

impl AnimationFrame {
    pub fn new() -> Self {
        AnimationFrame {}
    }
}

impl Subscription for AnimationFrame {
    type Message = f64;

    fn subscribe(self, send: Send<Self::Message>) -> Unsubscribe {
        let closure = Rc::new(RefCell::new(None));
        let closure2 = closure.clone();
        let done = Rc::new(Cell::new(false));
        let done2 = done.clone();

        *closure2.borrow_mut() = Some(Closure::wrap(Box::new(move |f64| {
            send(f64);
            if !done.get() {
                request_animation_frame(closure.borrow().as_ref().unwrap_throw());
            }
        }) as Box<dyn FnMut(f64)>));

        request_animation_frame(closure2.borrow().as_ref().unwrap_throw());

        return Unsubscribe::new(move || {
            done2.set(true);
        });

        fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
            web::window()
                .unwrap_throw()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .unwrap_throw();
        }
    }
}
