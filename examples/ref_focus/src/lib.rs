use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys as web;

#[derive(Default, Debug)]
pub struct RefFocus {
    show: bool,
    ref_: Option<web::Element>,
}

pub enum Message {
    Focus,
    Ref(Option<web::Element>),
    Toggle,
}

impl draco::Application for RefFocus {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Focus => {
                if let Some(ref ref_) = self.ref_ {
                    ref_.dyn_ref::<web::HtmlElement>()
                        .unwrap_throw()
                        .focus()
                        .unwrap_throw();
                }
            }
            Message::Ref(ref_) => self.ref_ = ref_,
            Message::Toggle => self.show = !self.show,
        }
    }

    fn view(&self) -> draco::VNode<Self::Message> {
        use draco::html as h;
        h::div()
            .with((
                if self.show {
                    h::input().ref_(Message::Ref)
                } else {
                    h::span()
                },
                h::button().with("Toggle").on("click", |_| Message::Toggle),
                h::button().with("Focus").on("click", |_| Message::Focus),
                h::pre().with(format!("{:#?}", self)),
            ))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        RefFocus::default(),
        draco::select("main").expect("<main>").into(),
    );
}
