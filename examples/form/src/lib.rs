use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
pub struct Form {
    username: String,
    password: String,
    accept: bool,
    is_submitting: bool,
}

pub enum Message {
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateAccept(bool),
    Submit,
    Notify,
}

impl draco::App for Form {
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

    fn render(&self) -> draco::Node<Self::Message> {
        use draco::html as h;
        h::form()
            .on("submit", |event| {
                event.prevent_default();
                Message::Submit
            })
            .push(h::pre().push(format!("{:#?}", self)))
            .push(h::label().for_("username").push("Username: "))
            .push(
                h::input()
                    .id("username")
                    .name("username")
                    .value(self.username.clone())
                    .on_input(Message::UpdateUsername),
            )
            .push(
                h::button()
                    .type_("button")
                    .push("Clear")
                    .on("click", |_| Message::UpdateUsername("".into())),
            )
            .push(h::br())
            .push(h::label().for_("password").push("Password: "))
            .push(
                h::input()
                    .id("password")
                    .name("password")
                    .type_("password")
                    .value(self.password.clone())
                    .on_input(Message::UpdatePassword),
            )
            .push(
                h::button()
                    .type_("button")
                    .push("Clear")
                    .on("click", |_| Message::UpdatePassword("".into())),
            )
            .push(h::br())
            .push(h::label().for_("accept").push("Accept "))
            .push(
                h::input()
                    .id("accept")
                    .name("accept")
                    .type_("checkbox")
                    .checked(self.accept)
                    .on_checked(Message::UpdateAccept),
            )
            .push(
                h::button()
                    .type_("button")
                    .disabled(self.accept)
                    .push("Agree")
                    .on("click", |_| Message::UpdateAccept(true)),
            )
            .push(
                h::button()
                    .type_("button")
                    .disabled(!self.accept)
                    .push("Disagree")
                    .on("click", |_| Message::UpdateAccept(false)),
            )
            .push(h::br())
            .push(if self.is_submitting {
                h::button().push("Submitting...").disabled(true)
            } else {
                h::button().push("Submit").disabled(!self.accept)
            })
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
