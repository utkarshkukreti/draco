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
