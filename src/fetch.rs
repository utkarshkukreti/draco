use futures::Future;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys as web;

#[derive(Debug)]
pub struct Request {
    method: String,
    url: String,
    body: Option<JsValue>,
    content_type: Option<&'static str>
}

impl Request {
    pub fn new(method: &str, url: &str) -> Request {
        Request {
            method: method.into(),
            url: url.into(),
            body: None,
            content_type: None
        }
    }
    pub fn with_body(method: &str, url: &str, body: JsValue, content_type: &'static str) -> Request {
        Request {
            method: method.into(),
            url: url.into(),
            body: Some(body),
            content_type: Some(content_type),
        }
    }

    pub fn send<R: Response>(self) -> impl Future<Item = R::Item, Error = Error> {
        let mut init = web::RequestInit::new();
        init.method(&self.method);
        if let Some(body) = self.body.as_ref() {
            init.body(Some(body));
        }

        let request = web::Request::new_with_str_and_init(&self.url, &init).unwrap();
        if let Some(content_type) = self.content_type {
            request.headers().set("Content-Type", content_type).unwrap();
        }
        let promise = web::window().unwrap().fetch_with_request(&request);
        R::send(JsFuture::from(promise).map(|response| {
            assert!(response.is_instance_of::<web::Response>());
            response.dyn_into::<web::Response>().unwrap()
        }))
    }
}

#[derive(Debug)]
pub struct Error(JsValue);

pub trait Response {
    type Item;

    fn send(
        future: impl Future<Item = web::Response, Error = JsValue> + 'static,
    ) -> Box<Future<Item = Self::Item, Error = Error>>;
}

#[derive(Debug)]
pub struct Text;

impl Response for Text {
    type Item = String;

    fn send(
        future: impl Future<Item = web::Response, Error = JsValue> + 'static,
    ) -> Box<Future<Item = Self::Item, Error = Error>> {
        Box::new(
            future
                .and_then(|response| response.text())
                .and_then(JsFuture::from)
                .map(|text| text.as_string().unwrap())
                .map_err(Error),
        )
    }
}

pub fn get(url: &str) -> Request {
    Request::new("GET", url)
}

pub fn post(url: &str) -> Request {
    Request::new("POST", url)
}

pub fn post_json(url: &str, json: &str) -> Request {
    let body = JsValue::from_str(json);
    Request::with_body("POST", url, body, "application/json")
}
