
# Draco

> Draco is a Rust library to build client side web applications with Web
> Assembly.

> [Live Examples with Annotated Source](https://draco-examples.netlify.com/) |
> [Starter](https://github.com/utkarshkukreti/draco-starter)

## Overview

The "Hello, World!" of Draco looks like this:

```rust
use wasm_bindgen::prelude::*;

struct HelloWorld;

impl draco::Application for HelloWorld {
    type Message = ();

    fn view(&self) -> draco::Node<Self::Message> {
        draco::html::h1().push("Hello, world!").into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(HelloWorld, draco::select("main").expect("<main>").into());
}
```

and the popular "Counter" application:

```rust
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

impl draco::Application for Counter {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
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

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        Counter::default(),
        draco::select("main").expect("<main>").into(),
    );
}
```

## Explanation

Here are the two same examples above, with comments explaining everything.

```rust
// examples/hello_world/src/lib.rs
use wasm_bindgen::prelude::*;

// A Draco application is a struct (or enum) which stores its state.
// We declare a unit struct here because we don't have any state to store.
struct HelloWorld;

// A Draco application must implement the `draco::Application` trait.
impl draco::Application for HelloWorld {
    // `Message` is the type of value our HTML will emit.
    // Here we aren't emitting anything so we use the unit type.
    // You can put any type here and this example will still compile.
    type Message = ();

    // The `view` function returns what we want to display on the page.
    fn view(&self) -> draco::Node<Self::Message> {
        // `draco::html::h1()` creates an `<h1>` element.
        draco::html::h1()
            // `.push()` adds a child to the element. Here we add a Text Node by pushing a string.
            .push("Hello, world!")
            // We use `.into()` to convert an `Element` struct to a `Node` struct which this
            // function must return.
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // We select the first element on the page matching the CSS selector `main` and start the
    // application on it.
    draco::start(HelloWorld, draco::select("main").expect("<main>").into());
}
```

```rust
// examples/counter/src/lib.rs
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

    fn view(&self) -> draco::Node<Self::Message> {
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
```
