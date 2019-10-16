use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys as web;

#[derive(Default)]
struct MouseTracker {
    x: i32,
    y: i32,
    subscription: Option<draco::Unsubscribe>,
}

enum Message {
    Toggle,
    Move(web::MouseEvent),
}

impl draco::App for MouseTracker {
    type Message = Message;

    fn update(&mut self, message: Self::Message, mailbox: &draco::Mailbox<Message>) {
        match message {
            Message::Toggle => {
                if self.subscription.take().is_none() {
                    self.subscription = Some(
                        mailbox.subscribe(draco::subscription::OnWindow::new("mousemove"), |ev| {
                            Message::Move(ev.dyn_into().unwrap())
                        }),
                    );
                }
            }
            Message::Move(mouse_event) => {
                self.x = mouse_event.screen_x();
                self.y = mouse_event.screen_y();
            }
        }
    }

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::div()
            .push(h::h1().push(if self.subscription.is_some() {
                "Tracking"
            } else {
                "Not Tracking"
            }))
            .push(
                h::button()
                    .push(if self.subscription.is_some() {
                        "Stop"
                    } else {
                        "Start"
                    })
                    .on("click", |_| Message::Toggle),
            )
            .push(
                h::div()
                    .push("x = ")
                    .push(self.x)
                    .push("; ")
                    .push("y = ")
                    .push(self.y),
            )
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        MouseTracker::default(),
        draco::select("main").expect("<main>").into(),
    );
}
