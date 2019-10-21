use draco::router::Mode::Hash;
use draco::url::Url;
use wasm_bindgen::prelude::*;

struct Router {
    url: Url,
}

impl Router {
    fn new() -> Self {
        Router {
            url: draco::router::current_url(Hash),
        }
    }
}

enum Message {
    Navigate(Url),
    NoOp,
}

impl draco::NoOp for Message {
    fn noop() -> Self {
        Message::NoOp
    }
}

impl draco::Application for Router {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _mailbox: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Navigate(url) => {
                self.url = url;
            }
            Message::NoOp => {}
        }
    }

    fn view(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        let urls = [
            "/",
            "/foo",
            "/foo/bar",
            "/foo/bar?baz=quux",
            "/foo/bar?baz#quux",
        ];

        h::div()
            .push(h::h3().push(format!("Current Url: {:?}", &self.url)))
            .append(urls.iter().map(|url| {
                h::div()
                    .attribute(
                        "style",
                        if Url::from(*url) == self.url {
                            "padding: .25rem .75rem; background: #fefcbf; border: 1px solid #ecc94b; border-radius: 4px;"
                        } else {
                            "padding: .25rem .75rem;"
                        },
                    )
                    .push(
                        h::span().push(
                            draco::router::link(Hash, Url::from(*url))
                                .push(url.to_string())
                                .attribute("style", "margin-right: .5rem;"),
                        ),
                    )
            }))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let mailbox = draco::start(Router::new(), draco::select("main").expect("<main>").into());

    mailbox.subscribe_forever(draco::router::Router::new(Hash), Message::Navigate);
}
