use draco::url::Url;
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct Router {
    page: Page,
}

enum Message {
    Navigate(Page),
    NoOp,
}

impl Default for Message {
    fn default() -> Self {
        Message::NoOp
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Page {
    Index,
    PostIndex { sort: Option<String> },
    PostShow { id: i32, hash: Option<String> },
    NotFound,
}

impl Default for Page {
    fn default() -> Self {
        Page::Index
    }
}

impl draco::router::Route for Page {
    fn from_url(url: Url) -> Self {
        use draco::url::parse::*;
        parse(&url)
            // /
            .alt((), |()| Page::Index)
            // /posts
            // /posts?sort=some-string
            .alt(("posts", query("sort").optional()), |((), sort)| {
                Page::PostIndex { sort }
            })
            // /posts/123
            // /posts/123#some-string
            .alt(("posts", param()), |((), id)| Page::PostShow {
                id,
                hash: url.hash().clone(),
            })
            .value()
            .unwrap_or(Page::NotFound)
    }

    fn to_url(&self) -> Url {
        let root = draco::url::build();
        match self {
            Page::Index => root,
            Page::PostIndex { sort } => root.path("posts").query_optional("sort", sort.as_ref()),
            Page::PostShow { id, hash } => root.path("posts").path(id).hash(hash.as_ref()),
            Page::NotFound => root,
        }
        .finish()
    }
}

impl draco::App for Router {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _mailbox: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
            Message::NoOp => {}
        }
    }

    fn view(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        use draco::router::Mode::Hash;
        let pages = [
            Page::Index,
            Page::PostIndex { sort: None },
            Page::PostIndex {
                sort: Some("id".into()),
            },
            Page::PostIndex {
                sort: Some("title".into()),
            },
            Page::PostShow { id: 1, hash: None },
            Page::PostShow {
                id: 1,
                hash: Some("section-1".into()),
            },
            Page::PostShow { id: 2, hash: None },
            Page::PostShow {
                id: 2,
                hash: Some("section-1".into()),
            },
        ];

        h::div()
            .push(h::h3().push(format!("Current Page: {:?}", &self.page)))
            .append(pages.iter().map(|page| {
                h::div()
                    .attribute(
                        "style",
                        if page == &self.page {
                            "padding: .25rem .75rem; background: #fefcbf; border: 1px solid #ecc94b; border-radius: 4px;"
                        } else {
                            "padding: .25rem .75rem;"
                        },
                    )
                    .push(
                        h::span().push(
                            draco::router::link(Hash, page.clone())
                                .push(format!("{:?}", page))
                                .attribute("style", "margin-right: .5rem;"),
                        ),
                    )
                    .push(
                        h::button()
                            .push("Push")
                            .on("click", {
                                let page = page.clone();
                                move |_| {
                                    draco::router::push(Hash, &page);
                                    Message::NoOp
                                }
                            })
                            .attribute("style", "margin-right: .5rem;"),
                    )
                    .push(h::button().push("Replace").on("click", {
                        let page = page.clone();
                        move |_| {
                            draco::router::replace(Hash, &page);
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
        draco::select("main").expect("<main>").into(),
    );

    mailbox.subscribe_forever(
        draco::router::Router::new(draco::router::Mode::Hash),
        Message::Navigate,
    );
}
