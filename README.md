# Draco

> Draco is a Rust library to build client side web applications with Web
> Assembly.

> [Live Examples](https://draco-examples.netlify.com/) |
> [Starter](https://github.com/utkarshkukreti/draco-starter)

## Overview

The "Hello, World!" of Draco looks like this:

```rust
use wasm_bindgen::prelude::*;

// A Draco application requires a struct to store its state.
// We declare a unit struct here because we don't have any state to store.
struct HelloWorld;

// A Draco application must implement the `draco::App` trait.
impl draco::App for HelloWorld {
    // `Message` is the type of value our HTML will emit.
    // Here we aren't emitting anything so we use the unit type.
    // You can put any type here and this will still compile.
    type Message = ();

    // The `view` function returns what we want to display on the page.
    fn view(&self) -> draco::Node<Self::Message> {
        // `draco::html` contains functions to create HTML elements.
        // `draco::html::h1()` creates an `<h1>` element.
        // `.push()` adds a child to the element. Here we add a Text child by pushing a string.
        // We use `.into()` at the end to convert an `Element` struct to a `Node` struct which
        // this function must return.
        draco::html::h1().push("Hello, world!").into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // We select the first `<main>` in the page and start running the application on it.
    draco::start(HelloWorld, draco::select("main").expect("<main>").into());
}
```

The introduction of any frontend framework is incomplete without the "Counter"
example, so here it goes:

```rust
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
```
