use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Form {
    username: String,
    password: String,
    accept: bool,
    plan: String,
    is_submitting: bool,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            username: "".into(),
            password: "".into(),
            accept: false,
            plan: "C3".into(),
            is_submitting: false,
        }
    }
}

pub enum Message {
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateAccept(bool),
    UpdatePlan(String),
    Submit,
    Notify,
}

impl draco::Application for Form {
    type Message = Message;

    fn update(&mut self, message: Self::Message, mailbox: &draco::Mailbox<Self::Message>) {
        use self::Message::*;
        match message {
            UpdateUsername(username) => {
                self.username = username;
            }
            UpdatePassword(password) => {
                self.password = password;
            }
            UpdateAccept(accept) => {
                self.accept = accept;
            }
            UpdatePlan(plan) => {
                self.plan = plan;
            }
            Submit => {
                self.is_submitting = true;
                mailbox.send_after(1000, || Notify);
            }
            Notify => {
                self.is_submitting = false;
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&format!("Submitted: {:#?}", self))
                    .unwrap();
            }
        }
    }

    fn view(&self) -> draco::VNode<Self::Message> {
        use draco::html as h;
        let plans = ["A1", "B2", "C3", "D4", "E5"];
        h::form()
            .on("submit", |event| {
                event.prevent_default();
                Message::Submit
            })
            .with((
                h::pre().with(format!("{:#?}", self)),
                h::label().for_("username").with("Username: "),
                h::input()
                    .id("username")
                    .name("username")
                    .value(self.username.clone())
                    .on_input(Message::UpdateUsername),
                h::button()
                    .type_("button")
                    .with("Clear")
                    .on("click", |_| Message::UpdateUsername("".into())),
                h::br(),
                h::label().for_("password").with("Password: "),
                h::input()
                    .id("password")
                    .name("password")
                    .type_("password")
                    .value(self.password.clone())
                    .on_input(Message::UpdatePassword),
                h::button()
                    .type_("button")
                    .with("Clear")
                    .on("click", |_| Message::UpdatePassword("".into())),
                h::br(),
                h::div().with(h::label().for_("plan").with("Plan")).with(
                    h::select()
                        .value(self.plan.clone())
                        .on_input(Message::UpdatePlan)
                        .append(plans.iter().map(|plan| {
                            h::option().value(plan.to_string()).with(plan.to_string())
                        })),
                ),
                h::label().for_("accept").with("Accept "),
                h::input()
                    .id("accept")
                    .name("accept")
                    .type_("checkbox")
                    .checked(self.accept)
                    .on_checked(Message::UpdateAccept),
                h::button()
                    .type_("button")
                    .disabled(self.accept)
                    .with("Agree")
                    .on("click", |_| Message::UpdateAccept(true)),
                h::button()
                    .type_("button")
                    .disabled(!self.accept)
                    .with("Disagree")
                    .on("click", |_| Message::UpdateAccept(false)),
                h::br(),
                if self.is_submitting {
                    h::button().with("Submitting...").disabled(true)
                } else {
                    h::button().with("Submit").disabled(!self.accept)
                },
            ))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        Form::default(),
        draco::select("main").expect("<main>").into(),
    );
}
