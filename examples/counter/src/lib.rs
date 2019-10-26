use wasm_bindgen::prelude::*;

// This is our state -- just a `value` of type `i32`.
#[derive(Default)]
pub struct Counter {
    value: i32,
}

// Our app's state can be updated in three ways:
pub enum Message {
    // Clicking `+`; adds 1 to `value`.
    Increment,
    // Clicking `-`; subtracts 1 from `value`.
    Decrement,
    // Clicking `Reset`; sets `value` to 0.
    Reset,
}

impl draco::Application for Counter {
    // This is the type our `view` will emit and `update` will handle.
    type Message = Message;

    fn update(&mut self, message: Self::Message, _: &draco::Mailbox<Self::Message>) {
        // We simply update `self.value` for the three possible messages.
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> draco::VNode<Self::Message> {
        use draco::html as h;
        h::div()
            .push(
                h::button()
                    .push("-")
                    // `.on` adds an event listener to the element.
                    .on(
                        // `click` is the event we want to listen to.
                        "click",
                        // The closure takes one argument of `web_sys::Event` type.
                        // We don't need it here so we ignore it with `_`.
                        |_| {
                            // The closure returns the message we want our `update` function to
                            // receive when the event is triggered.
                            Message::Decrement
                        },
                    ),
            )
            .push(" ")
            .push(self.value)
            .push(" ")
            .push(h::button().push("+").on("click", |_| Message::Increment))
            .push(" ")
            .push(h::button().push("Reset").on("click", |_| Message::Reset))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Like in the `HelloWorld` example we start the application on the first element in the page
    // matching the selector `main`.
    draco::start(
        Counter::default(),
        draco::select("main").expect("<main>").into(),
    );
}
