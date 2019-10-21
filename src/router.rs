use crate::{subscription, NonKeyedElement, Subscription, Unsubscribe};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys as web;

use crate::url::Url;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Hash,
    History,
}

#[derive(Debug)]
pub struct Router<R: Route> {
    mode: Mode,
    phantom: std::marker::PhantomData<R>,
}

impl<R: Route> Router<R> {
    pub fn new(mode: Mode) -> Self {
        Router {
            mode,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<R: Route + 'static> Subscription for Router<R> {
    type Message = R;

    fn subscribe(self, send: subscription::Send<Self::Message>) -> Unsubscribe {
        let window = web::window().unwrap_throw();
        let closure = Closure::wrap(Box::new(move || {
            send(R::from_url(current_url(self.mode)));
        }) as Box<dyn FnMut()>);
        (window.as_ref() as &web::EventTarget)
            .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
            .unwrap_throw();
        (window.as_ref() as &web::EventTarget)
            .dispatch_event(&web::Event::new("popstate").unwrap_throw())
            .unwrap_throw();

        Unsubscribe::new(move || {
            (window.as_ref() as &web::EventTarget)
                .remove_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .unwrap_throw();
        })
    }
}

pub trait Route {
    fn from_url(url: Url) -> Self;
    fn to_url(&self) -> Url;
}

impl Route for Url {
    fn from_url(url: Url) -> Self {
        url
    }

    fn to_url(&self) -> Url {
        self.clone()
    }
}

pub fn link<Message: crate::NoOp + 'static, R: Route + 'static>(
    mode: Mode,
    r: R,
) -> NonKeyedElement<Message> {
    crate::html::a()
        .href(href(mode, &r.to_url().to_string()))
        .on("click", move |event| {
            event.prevent_default();
            push(mode, &r);
            Message::noop()
        })
}

pub fn push<R: Route>(mode: Mode, r: &R) {
    web::window()
        .unwrap_throw()
        .history()
        .unwrap_throw()
        .push_state_with_url(
            &JsValue::NULL,
            "",
            Some(&href(mode, &r.to_url().to_string())),
        )
        .unwrap_throw();
    popstate();
}

pub fn replace<R: Route>(mode: Mode, r: &R) {
    web::window()
        .unwrap_throw()
        .history()
        .unwrap_throw()
        .replace_state_with_url(
            &JsValue::NULL,
            "",
            Some(&href(mode, &r.to_url().to_string())),
        )
        .unwrap_throw();
    popstate();
}

pub fn current_url(mode: Mode) -> Url {
    let location = web::window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .location()
        .unwrap_throw();
    Url::from(match mode {
        Mode::Hash => {
            let mut hash = location.hash().unwrap_throw();
            if !hash.is_empty() {
                hash.remove(0);
            }
            hash
        }
        Mode::History => {
            location.pathname().unwrap_throw()
                + &location.search().unwrap_throw()
                + &location.hash().unwrap_throw()
        }
    })
}

fn href(mode: Mode, href: &str) -> String {
    match mode {
        Mode::Hash => String::from("#") + href,
        Mode::History => href.to_string(),
    }
}

fn popstate() {
    (web::window().unwrap_throw().as_ref() as &web::EventTarget)
        .dispatch_event(&web::Event::new("popstate").unwrap_throw())
        .unwrap_throw();
}
