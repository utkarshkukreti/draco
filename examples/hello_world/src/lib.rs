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
    fn view(&self) -> draco::VNode<Self::Message> {
        // `draco::html::h1()` creates an `<h1>` element.
        draco::html::h1()
            // `.with()` adds a child to the element. Here we add a Text Node by pushing a string.
            .with("Hello, world!")
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
