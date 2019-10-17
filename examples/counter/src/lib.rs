use wasm_bindgen::prelude::*;

// This is our state. Just a `value` of type `i32`.
#[derive(Default)]
pub struct Counter {
    value: i32,
}

// Our app's state can be updated for three reasons:
pub enum Message {
    // Clicking `+`
    Increment,
    // Clicking `-`
    Decrement,
    // Clicking `Reset`
    Reset,
}

impl draco::App for Counter {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _: &draco::Mailbox<Self::Message>) {
        // We simply update `self.value` for the three possible messages.
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
            // `.on` adds an event listener to the element.
            // "click" is the event we want to listen
            // The closure returns the message we want our `update` function to receive.
            // The closure takes one argument of `web_sys::Event` type. We don't need it here so
            // we ignore it with `_`.
            .push(h::button().push("-").on("click", |_| Message::Decrement))
            .push(" ")
            .push(self.value)
            .push(" ")
            .push(h::button().push("+").on("click", |_| Message::Increment))
            .push(" ")
            .push(h::button().push("Reset").on("click", |_| Message::Reset))
            .into()
    }
}

// Like in the `HelloWorld` example we start the application on the first `<main>` in the page.
#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        Counter::default(),
        draco::select("main").expect("<main>").into(),
    );
}
