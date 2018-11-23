use rand::{
    prng::XorShiftRng,
    {Rng, SeedableRng},
};
use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use web_sys as web;

#[wasm_bindgen]
pub fn start() {
    let non_keyed = web::window()
        .unwrap()
        .location()
        .href()
        .unwrap()
        .contains("non-keyed");
    draco::start(
        Jfb::new(!non_keyed),
        draco::select("main").expect("main").into(),
    );
}

pub struct Jfb {
    rows: Vec<Tracked<Row>>,
    next_id: usize,
    selected_id: Option<usize>,
    rng: XorShiftRng,
    keyed: bool,
}

struct Row {
    id: usize,
    label: String,
}

impl Row {
    fn new<R: Rng>(id: usize, rng: &mut R) -> Row {
        let label = format!(
            "{} {} {}",
            rng.choose(ADJECTIVES).unwrap(),
            rng.choose(COLORS).unwrap(),
            rng.choose(NOUNS).unwrap()
        );

        Row { id, label }
    }

    fn render<Message>(
        &self,
        is_changed: bool,
        selected_id: Option<usize>,
    ) -> draco::Node<Message> {
        use draco::html as h;
        let mut tr = h::tr().class(if selected_id == Some(self.id) {
            "danger"
        } else {
            ""
        });
        if is_changed {
            tr.push(h::td().class("col-md-1").push(self.id))
                .push(
                    h::td()
                        .class("col-md-4")
                        .push(h::a().class("lbl").push(&self.label)),
                )
                .push(
                    h::td().class("col-md-1").push(
                        h::a()
                            .class("remove")
                            .push(
                                h::span()
                                    .class("glyphicon glyphicon-remove remove")
                                    .attr("aria-hidden", "true"),
                            )
                            .push(h::td().class("col-md-6")),
                    ),
                )
                .into()
        } else {
            tr.cache_children();
            tr.into()
        }
    }
}

#[derive(Clone)]
pub enum Message {
    Create(usize),
    Append(usize),
    UpdateEvery(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
    NoOp,
}

impl Jfb {
    pub fn new(keyed: bool) -> Self {
        Jfb {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: XorShiftRng::from_seed([0; 16]),
            keyed,
        }
    }

    fn on_click(event: web::Event) -> Message {
        use wasm_bindgen::JsCast;
        let target = event.target().unwrap();
        let target: web_sys::Element = target.dyn_into().unwrap();
        if target.matches(".remove").unwrap() || target.matches(".lbl").unwrap() {
            let td: web_sys::Node = target
                .closest("tr")
                .unwrap()
                .unwrap()
                .query_selector("td")
                .unwrap()
                .unwrap()
                .into();
            let id = td.text_content().unwrap().parse().unwrap();
            if target.matches(".remove").unwrap() {
                Message::Remove(id)
            } else {
                Message::Select(id)
            }
        } else {
            Message::NoOp
        }
    }

    fn buttons() -> impl Iterator<Item = draco::Node<Message>> {
        use draco::html as h;

        struct Button {
            id: &'static str,
            message: Message,
            description: &'static str,
        }

        static BUTTONS: &[Button] = &[
            Button {
                id: "run",
                description: "Create 1,000 rows",
                message: Message::Create(1000),
            },
            Button {
                id: "runlots",
                description: "Create 10,000 rows",
                message: Message::Create(10000),
            },
            Button {
                id: "add",
                description: "Append 1,000 rows",
                message: Message::Append(1000),
            },
            Button {
                id: "update",
                description: "Update every 10th row",
                message: Message::UpdateEvery(10),
            },
            Button {
                id: "clear",
                description: "Clear",
                message: Message::Clear,
            },
            Button {
                id: "swaprows",
                description: "Swap Rows",
                message: Message::Swap,
            },
        ];

        BUTTONS.iter().map(|button| {
            h::div()
                .class("col-sm-6 smallpad")
                .push(
                    h::button()
                        .attr("id", button.id)
                        .class("btn btn-primary btn-block")
                        .attr("type", "button")
                        .on("click", move |_| button.message.clone())
                        .push(button.description),
                )
                .into()
        })
    }
}

impl draco::App for Jfb {
    type Message = Message;

    fn update(&mut self, mailbox: &draco::Mailbox<Message>, message: Self::Message) {
        let Jfb {
            next_id,
            rng,
            rows,
            selected_id,
            ..
        } = self;
        match message {
            Message::Create(amount) => {
                rows.clear();
                self.update(mailbox, Message::Append(amount));
            }
            Message::Append(amount) => {
                rows.extend((0..amount).map(|index| Tracked::new(Row::new(*next_id + index, rng))));
                *next_id += amount;
            }
            Message::UpdateEvery(step) => {
                for index in (0..rows.len()).step_by(step) {
                    rows[index].label += " !!!";
                }
            }
            Message::Clear => {
                rows.clear();
            }
            Message::Swap => {
                if rows.len() > 998 {
                    rows.swap(1, 998);
                }
            }
            Message::Remove(id) => {
                if let Some((index, _)) = rows.iter().enumerate().find(|(_, row)| row.id == id) {
                    rows.remove(index);
                }
            }
            Message::Select(id) => {
                if *selected_id == Some(id) {
                    *selected_id = None;
                } else {
                    *selected_id = Some(id);
                }
            }
            Message::NoOp => {}
        }
    }

    fn render(&self) -> draco::Node<Message> {
        use draco::html as h;

        h::div()
            .class("container")
            .push(
                h::div().class("jumbotron").push(
                    h::div()
                        .class("row")
                        .push(h::div().class("col-md-6").push(h::h1().push("Draco")))
                        .push(h::div().class("col-md-6").append(Self::buttons())),
                ),
            )
            .push(
                h::table()
                    .on("click", Self::on_click)
                    .class("table table-hover table-striped test-data")
                    .push({
                        let node: draco::Node<Message> = if self.keyed {
                            draco::html::keyed::tbody()
                                .attr("id", "tbody")
                                .append(self.rows.iter().map(|row| {
                                    (
                                        row.id as u64,
                                        row.render(row.is_changed(), self.selected_id),
                                    )
                                }))
                                .into()
                        } else {
                            h::tbody()
                                .attr("id", "tbody")
                                .append(
                                    self.rows
                                        .iter()
                                        .map(|row| row.render(row.is_changed(), self.selected_id)),
                                )
                                .into()
                        };
                        node
                    }),
            )
            .push(
                h::span()
                    .class("preloadicon glyphicon glyphicon-remove")
                    .attr("aria-hidden", "true"),
            )
            .into()
    }
}

static ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLORS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn main() {}

pub struct Tracked<T> {
    value: T,
    changed: Cell<bool>, //actual Tracked should use HashSet or something else, so there could be many readers
}

impl<T> Tracked<T> {
    pub fn new(t: T) -> Self {
        Tracked {
            value: t,
            changed: Cell::new(true),
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
