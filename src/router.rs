use crate::{subscription, NonKeyedElement, Subscription, Unsubscribe};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys as web;

mod parse;
mod url;

pub use self::parse::{hash, param, parse, query, Parse};
pub use self::url::Url;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Hash,
    History,
}

#[derive(Debug)]
pub struct Router {
    mode: Mode,
}

impl Router {
    pub fn new(mode: Mode) -> Self {
        Router { mode }
    }
}

impl Subscription for Router {
    type Message = Url;

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
            send(Url::new(url));
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

pub fn link<Message: Default + 'static>(mode: Mode, url: &str) -> NonKeyedElement<Message> {
    let url = url.to_string();
    crate::html::a()
        .attr("href", url.clone())
        .on("click", move |event| {
            event.prevent_default();
            push(mode, &url);
            Message::default()
        })
}

pub fn push(mode: Mode, url: &str) {
    web::window()
        .unwrap()
        .history()
        .unwrap()
        .push_state_with_url(&JsValue::NULL, "", Some(&href(mode, url)))
        .unwrap();
    popstate();
}

pub fn replace(mode: Mode, url: &str) {
    web::window()
        .unwrap()
        .history()
        .unwrap()
        .replace_state_with_url(&JsValue::NULL, "", Some(&href(mode, url)))
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
