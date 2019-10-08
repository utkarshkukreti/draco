use wasm_bindgen::prelude::*;

#[derive(Default)]
struct Router {
    url: Option<draco::router::Url>,
    route: Option<Route>,
    subscription: Option<draco::Unsubscribe>,
}

enum Message {
    Subscribe,
    Navigate(draco::router::Url),
    NoOp,
}

impl Default for Message {
    fn default() -> Self {
        Message::NoOp
    }
}

#[derive(Debug)]
enum Route {
    Index,
    PostIndex { sort: Option<String> },
    PostShow { id: i32, hash: Option<String> },
}

impl Route {
    fn new(url: &draco::router::Url) -> Option<Self> {
        use draco::router::*;
        parse(url)
            // /
            .alt((), |()| Route::Index)
            // /posts
            // /posts?sort=some-string
            .alt(("posts", query("sort").optional()), |((), sort)| {
                Route::PostIndex { sort }
            })
            // /posts/123
            // /posts/123#some-string
            .alt(("posts", param()), |((), id)| Route::PostShow {
                id,
                hash: url.hash.clone(),
            })
            .value()
    }
}

impl draco::App for Router {
    type Message = Message;

    fn update(&mut self, mailbox: &draco::Mailbox<Message>, message: Self::Message) {
        match message {
            Message::Subscribe => {
                self.subscription = Some(mailbox.subscribe(
                    draco::router::Router::new(draco::router::Mode::Hash),
                    Message::Navigate,
                ));
            }
            Message::Navigate(url) => {
                self.route = Route::new(&url);
                self.url = Some(url);
            }
            Message::NoOp => {}
        }
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        use draco::router::Mode::Hash;
        let links = [
            "/",
            "/posts",
            "/posts?sort=id",
            "/posts?sort=title",
            "/posts/1",
            "/posts/1#section-1",
            "/posts/2",
            "/posts/2#section-1",
        ];

        h::div()
            .push(h::div().push(format!("Url: {:?}", &self.url)))
            .push(h::div().push(format!("Route: {:?}", &self.route)))
            .append(links.iter().map(|link| {
                h::h2()
                    .push(
                        h::span().push(
                            draco::router::link(Hash, link)
                                .push(link.to_string())
                                .attr("style", "margin-right: .5rem;"),
                        ),
                    )
                    .push(h::button().push("Push").on("click", {
                        let link = link.clone();
                        move |_| {
                            draco::router::push(Hash, link);
                            Message::NoOp
                        }
                    }))
                    .push(h::button().push("Replace").on("click", {
                        let link = link.clone();
                        move |_| {
                            draco::router::replace(Hash, link);
                            Message::NoOp
                        }
                    }))
            }))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let mailbox = draco::start(
        Router::default(),
        draco::select("main").expect("main").into(),
    );
    mailbox.send(Message::Subscribe);
}
