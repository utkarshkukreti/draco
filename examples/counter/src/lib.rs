use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

pub enum Message {
    Increment,
    Decrement,
    Reset,
}

impl draco::App for Counter {
    type Message = Message;

    fn update(&mut self, _: &draco::Mailbox<Message>, message: Self::Message) {
        use self::Message::*;
        match message {
            Increment => self.value += 1,
            Decrement => self.value -= 1,
            Reset => self.value = 0,
        }
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
            .push(h::button().push("-").on("click", |_| Message::Decrement))
            .push(self.value)
            .push(h::button().push("+").on("click", |_| Message::Increment))
            .push(h::button().push("Reset").on("click", |_| Message::Reset))
            .into()
    }
}

#[wasm_bindgen]
pub fn start() {
    draco::start(
        Counter::default(),
        draco::select("main").expect("main").into(),
    );
}

pub fn main() {}
