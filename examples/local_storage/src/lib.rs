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

impl draco::Application for LocalStorage {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Update(value) => {
                Self::storage().set_item(KEY, &value).expect("set_item");
                self.value = value;
            }
        }
    }

    fn view(&self) -> draco::VNode<Self::Message> {
        use draco::html as h;
        h::div()
            .with(h::p().with("Type anything below."))
            .with(h::p().with(
                "The value is automatically stored in LocalStorage and restored on page load.",
            ))
            .with(
                h::textarea()
                    .value(self.value.clone())
                    .on_input(Message::Update),
            )
            .with(
                h::button()
                    .with("Clear")
                    .on("click", |_| Message::Update("".into())),
            )
            .with(h::pre().with(format!("{:?}", self)))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        LocalStorage::new(),
        draco::select("main").expect("<main>").into(),
    );
}
