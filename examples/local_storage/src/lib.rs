use wasm_bindgen::prelude::*;

const KEY: &str = "draco.examples.local_storage";

#[derive(Debug)]
pub struct LocalStorage {
    value: String,
}

impl LocalStorage {
    fn new() -> Self {
        LocalStorage {
            value: Self::storage()
                .get_item(KEY)
                .expect("get_item")
                .unwrap_or("".into()),
        }
    }

    fn storage() -> web_sys::Storage {
        web_sys::window()
            .expect("window")
            .local_storage()
            .expect("window.local_storage")
            .expect("window.local_storage")
    }
}

pub enum Message {
    Update(String),
}

impl draco::App for LocalStorage {
    type Message = Message;

    fn update(&mut self, _: &draco::Mailbox<Message>, message: Self::Message) {
        match message {
            Message::Update(value) => {
                Self::storage().set_item(KEY, &value).expect("set_item");
                self.value = value;
            }
        }
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
            .push(h::p().push("Type anything below."))
            .push(h::p().push(
                "The value is automatically stored in LocalStorage and restored on page load.",
            ))
            .push(
                h::textarea()
                    .attr("value", self.value.clone())
                    .on_input(Message::Update),
            )
            .push(
                h::button()
                    .push("Clear")
                    .on("click", |_| Message::Update("".into())),
            )
            .push(h::pre().push(format!("{:?}", self)))
            .into()
    }
}

#[wasm_bindgen]
pub fn start() {
    draco::start(
        LocalStorage::new(),
        draco::select("main").expect("main").into(),
    );
}

pub fn main() {}
