use wasm_bindgen::prelude::*;

struct HelloWorld;

impl draco::App for HelloWorld {
    type Message = ();

    fn render(&self) -> draco::Node<Self::Message> {
        draco::html::h1().push("Hello, world!").into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(HelloWorld, draco::select("main").expect("<main>").into());
}
