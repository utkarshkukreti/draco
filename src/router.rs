use crate::{subscription, NonKeyedElement, Subscription, Unsubscribe};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys as web;

pub use crate::url::{
    parse::{hash, param, parse, query, Parse},
    Url,
};

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
        let window = web::window().unwrap();
        let mode = self.mode;
        let closure = Closure::wrap(Box::new(move || {
            let location = web::window()
                .unwrap()
                .document()
                .unwrap()
                .location()
                .unwrap();
            let url = match mode {
                Mode::Hash => {
                    let mut hash = location.hash().unwrap();
                    if !hash.is_empty() {
                        hash.remove(0);
                    }
                    hash
                }
                Mode::History => {
                    location.pathname().unwrap()
                        + &location.search().unwrap()
                        + &location.hash().unwrap()
                }
            };
            send(R::from_url(Url::from(url)));
        }) as Box<dyn FnMut()>);
        (window.as_ref() as &web::EventTarget)
            .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
            .unwrap();
        (window.as_ref() as &web::EventTarget)
            .dispatch_event(&web::Event::new("popstate").unwrap())
            .unwrap();

        Unsubscribe::new(move || {
            (window.as_ref() as &web::EventTarget)
                .remove_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .unwrap();
        })
    }
}

pub trait Route {
    fn from_url(url: Url) -> Self;
    fn to_url(&self) -> Url;
}

pub fn link<Message: Default + 'static, R: Route + 'static>(
    mode: Mode,
    r: R,
) -> NonKeyedElement<Message> {
    crate::html::a()
        .attr("href", r.to_url().to_string())
        .on("click", move |event| {
            event.prevent_default();
            push(mode, &r);
            Message::default()
        })
}

pub fn push<R: Route>(mode: Mode, r: &R) {
    web::window()
        .unwrap()
        .history()
        .unwrap()
        .push_state_with_url(
            &JsValue::NULL,
            "",
            Some(&href(mode, &r.to_url().to_string())),
        )
        .unwrap();
    popstate();
}

pub fn replace<R: Route>(mode: Mode, r: &R) {
    web::window()
        .unwrap()
        .history()
        .unwrap()
        .replace_state_with_url(
            &JsValue::NULL,
            "",
            Some(&href(mode, &r.to_url().to_string())),
        )
        .unwrap();
    popstate();
}

fn href(mode: Mode, href: &str) -> String {
    match mode {
        Mode::Hash => String::from("#") + href,
        Mode::History => href.to_string(),
    }
}

fn popstate() {
    (web::window().unwrap().as_ref() as &web::EventTarget)
        .dispatch_event(&web::Event::new("popstate").unwrap())
        .unwrap();
}
