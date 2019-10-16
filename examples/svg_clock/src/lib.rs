use js_sys as js;
use wasm_bindgen::prelude::*;

struct Clock {
    date: js::Date,
}

enum Message {
    Tick,
}

impl Clock {
    fn new() -> Self {
        Clock {
            date: js::Date::new_0(),
        }
    }
}

impl draco::App for Clock {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _mailbox: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Tick => {
                self.date = js::Date::new_0();
            }
        }
    }

    fn view(&self) -> draco::Node<Self::Message> {
        use draco::{html as h, svg as s};
        let circle = s::circle()
            .attribute("cx", "100")
            .attribute("cy", "100")
            .attribute("r", "98")
            .attribute("fill", "none")
            .attribute("stroke", "#1a202c");

        let line = |rotate: f64, stroke, stroke_width: u32, height: u32| {
            s::line()
                .attribute("x1", "100")
                .attribute("y1", "100")
                .attribute("x2", (100 - height).to_string())
                .attribute("y2", "100")
                .attribute("stroke", stroke)
                .attribute("stroke-width", stroke_width.to_string())
                .attribute("stroke-linecap", "round")
                .attribute(
                    "transform",
                    format!("rotate({} 100 100)", (rotate * 10.0).round() / 10.0),
                )
        };

        let d = &self.date;
        let ms = ((((d.get_hours() * 60 + d.get_minutes()) * 60) + d.get_seconds()) * 1000
            + d.get_milliseconds()) as f64;

        let subsecond_rotate = 90.0 + ((ms / 1000.0) % 1.0) * 360.0;
        let second_rotate = 90.0 + ((ms / 1000.0) % 60.0) * 360.0 / 60.0;
        let minute_rotate = 90.0 + ((ms / 1000.0 / 60.0) % 60.0) * 360.0 / 60.0;
        let hour_rotate = 90.0 + ((ms / 1000.0 / 60.0 / 60.0) % 12.0) * 360.0 / 12.0;

        h::div()
            .attribute(
                "style",
                "display: flex; align-items: center; flex-direction: column;",
            )
            .push(
                s::svg()
                    .attribute("width", "400")
                    .attribute("height", "400")
                    .attribute("viewBox", "0 0 200 200")
                    .push(circle)
                    .push(line(subsecond_rotate, "#e2e8f0", 10, 90))
                    .push(line(hour_rotate, "#2d3748", 4, 50))
                    .push(line(minute_rotate, "#2d3748", 3, 70))
                    .push(line(second_rotate, "#e53e3e", 2, 90)),
            )
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let mailbox = draco::start(Clock::new(), draco::select("main").expect("<main>").into());
    mailbox.stash(
        mailbox.subscribe(draco::subscription::AnimationFrame::new(), |()| {
            Message::Tick
        }),
    );
}
