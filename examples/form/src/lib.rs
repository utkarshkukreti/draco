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

    fn update(&mut self, mailbox: &draco::Mailbox<Message>, message: Self::Message) {
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
                    .alert_with_message(&format!("Submitted: {:?}", self))
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
            .push(h::pre().push(format!("{:?}", self)))
            .push(h::label().attr("for", "username").push("Username: "))
            .push(
                h::input()
                    .attr("id", "username")
                    .attr("name", "username")
                    .attr("value", self.username.clone())
                    .on_input(Message::UpdateUsername),
            )
            .push(
                h::button()
                    .attr("type", "button")
                    .push("Clear")
                    .on("click", |_| Message::UpdateUsername("".into())),
            )
            .push(h::br())
            .push(h::label().attr("for", "password").push("Password: "))
            .push(
                h::input()
                    .attr("id", "password")
                    .attr("name", "password")
                    .attr("type", "password")
                    .attr("value", self.password.clone())
                    .on_input(Message::UpdatePassword),
            )
            .push(
                h::button()
                    .attr("type", "button")
                    .push("Clear")
                    .on("click", |_| Message::UpdatePassword("".into())),
            )
            .push(h::br())
            .push(h::label().attr("for", "accept").push("Accept "))
            .push(
                h::input()
                    .attr("id", "accept")
                    .attr("name", "accept")
                    .attr("type", "checkbox")
                    .checked(self.accept)
                    .on_checked(Message::UpdateAccept),
            )
            .push(
                h::button()
                    .attr("type", "button")
                    .push("Agree")
                    .on("click", |_| Message::UpdateAccept(true)),
            )
            .push(
                h::button()
                    .attr("type", "button")
                    .push("Disagree")
                    .on("click", |_| Message::UpdateAccept(false)),
            )
            .push(h::br())
            .push(if self.is_submitting {
                h::button().push("Submitting...").attr("disabled", "")
            } else {
                h::button().push("Submit")
            })
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(Form::default(), draco::select("main").expect("main").into());
}
