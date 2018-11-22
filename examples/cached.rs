use wasm_bindgen::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    cell::Cell
};

struct HelloWorld {
    counter: u32,
    data: Tracked<Vec<&'static str>>,
    caching_enabled: bool
}

pub enum Message {
    Increment,
    EnableCaching
}

impl draco::App for HelloWorld {
    type Message = Message;

    fn update(&mut self, _: &draco::Mailbox<Message>, message: Self::Message) {
        use self::Message::*;
        match message {
            Increment => self.counter += 1,
            EnableCaching => self.caching_enabled = !self.caching_enabled
        };
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        let div = h::div()
            .push(h::button().push(self.counter).on("click", |_| Message::Increment))
            .push(h::button().push(format!{"Caching: {}", self.caching_enabled}).on("click", |_| Message::EnableCaching));
        let mut ul = h::ul();
        if !self.caching_enabled || self.data.is_changed() {
            for string in self.data.iter() {
                let li = h::li().push(&string);
                ul = ul.push(li);
            }
        } else {
            ul.cache_children()
        }
        div.push(ul).into()
    }
}

#[wasm_bindgen]
pub fn start() {
    draco::start(
        HelloWorld{
            data: Tracked::new(vec!["Hello world!"; 1000]),
            counter: 0,
            caching_enabled: false,
        },
        draco::select("main").expect("main").into()
    );
}

pub fn main() {}

pub struct Tracked<T> {
    value: T,
    changed: Cell<bool> //actual Tracked should use HashSet or something else, so there could be many readers
}

impl<T> Tracked<T> {
    pub fn new(t: T) -> Self {
        Tracked {
            value: t,
            changed: Cell::new(true)
        }
    }
    pub fn is_changed(&self) -> bool {
        if self.changed.get() {
            self.changed.set(false);
            true
        } else {
            false
        }
    }
}
impl<T> Deref for Tracked<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T> DerefMut for Tracked<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.changed.set(true);
        &mut self.value
    }
}