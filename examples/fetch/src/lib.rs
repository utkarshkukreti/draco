use wasm_bindgen::prelude::*;

type Response = Result<String, reqwest::Error>;

#[derive(Debug)]
pub struct Fetch {
    url: String,
    response: Option<Response>,
}

impl Fetch {
    fn new() -> Self {
        Fetch {
            url: "https://api.github.com/repos/rust-lang/rust/branches/master".into(),
            response: None,
        }
    }
}

pub enum Message {
    Send,
    UpdateResponse(Response),
    UpdateUrl(String),
}

impl draco::App for Fetch {
    type Message = Message;

    fn update(&mut self, mailbox: &draco::Mailbox<Message>, message: Self::Message) {
        use self::Message::*;
        match message {
            Send => match self.url.parse::<reqwest::Url>() {
                Ok(url) => mailbox.spawn(
                    async { Ok(reqwest::get(url).await?.text().await?) },
                    Message::UpdateResponse,
                ),
                Err(err) => draco::console::error(&err.to_string()),
            },
            UpdateResponse(response) => self.response = Some(response),
            UpdateUrl(url) => self.url = url,
        }
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
            .push(
                h::input()
                    .attr("value", self.url.clone())
                    .on_input(Message::UpdateUrl),
            )
            .push(h::button().push("GET").on("click", |_| Message::Send))
            .push(h::pre().push(format!("{:#?}", self)))
            .into()
    }
}

#[wasm_bindgen]
pub fn start() {
    draco::start(Fetch::new(), draco::select("main").expect("main").into());
}

pub fn main() {}
