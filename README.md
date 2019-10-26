
# Draco

> Draco is a Rust library to build client side web applications with Web
> Assembly.

> [Live Examples with Annotated Source](https://draco-examples.netlify.com/) |
> [Starter](https://github.com/utkarshkukreti/draco-starter)

> Note: This is the README for upcoming Draco 0.2.
> The README for the latest released version is
> [here](https://github.com/utkarshkukreti/draco/tree/0.1.2).

## Overview

The "Hello, World!" of Draco ([with comments here](examples/hello_world/src/lib.rs)):

```rust
use wasm_bindgen::prelude::*;

struct HelloWorld;

impl draco::Application for HelloWorld {
    type Message = ();

    fn view(&self) -> draco::VNode<Self::Message> {
        draco::html::h1().push("Hello, world!").into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(HelloWorld, draco::select("main").expect("<main>").into());
}
```

Draco is modeled after The Elm Architecture and Redux. A Draco application
implements the `draco::Application` trait, which includes one type and two
functions.

```rust
pub trait Application {
    type Message;

    fn update(&mut self, message: Self::Message, mailbox: &Mailbox<Self::Message>) {}
    fn view(&self) -> Node<Self::Message>;
}
```

The `view` function maps `&self` to an HTML/SVG Node. The Node can emit
`Message`s on certain events.

The `update` function takes `&mut self`, a `Message` and a `draco::Mailbox`.
This function may update its fields based on the value of the Message. This
function may also send more messages to itself or spawn a Future which will send
a message when it resolves, through the `Mailbox`.

### Counter

This Counter example ([with comments here](examples/counter/src/lib.rs))
demonstrates an application which updates an integer value based on 3 types of
messages emitted from the view.

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

    fn view(&self) -> draco::VNode<Self::Message> {
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
