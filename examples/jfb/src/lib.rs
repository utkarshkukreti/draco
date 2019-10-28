use rand::{
    prng::XorShiftRng,
    {Rng, SeedableRng},
};
use wasm_bindgen::prelude::*;
use web_sys as web;

#[wasm_bindgen(start)]
pub fn start() {
    let non_keyed = web::window()
        .unwrap()
        .location()
        .pathname()
        .unwrap()
        .contains("non-keyed");
    draco::start(
        Jfb::new(!non_keyed),
        draco::select("main").expect("<main>").into(),
    );
}

pub struct Jfb {
    rows: Vec<Row>,
    next_id: u32,
    selected_id: Option<u32>,
    rng: XorShiftRng,
    keyed: bool,
}

#[derive(Clone, Hash)]
struct Row {
    id: u32,
    label: String,
}

impl Row {
    fn new<R: Rng>(id: u32, rng: &mut R) -> Row {
        let label = format!(
            "{} {} {}",
            rng.choose(ADJECTIVES).unwrap(),
            rng.choose(COLORS).unwrap(),
            rng.choose(NOUNS).unwrap()
        );

        Row { id, label }
    }

    fn view(&self, is_selected: bool) -> draco::VNode<Message> {
        use draco::html as h;
        draco::Lazy::new((self.clone(), is_selected), |(row, is_selected)| {
            let id = row.id;
            h::tr()
                .class(if *is_selected { "danger" } else { "" })
                .with((
                    h::td().class("col-md-1").with(row.id),
                    h::td()
                        .class("col-md-4")
                        .on("click", move |_| Message::Select(id))
                        .with(h::a().with(row.label.clone())),
                    h::td().class("col-md-1").with(
                        h::a()
                            .class("remove")
                            .on("click", move |_| Message::Remove(id))
                            .with(
                                h::span()
                                    .class("glyphicon glyphicon-remove")
                                    .attribute("aria-hidden", "true"),
                            ),
                    ),
                    h::td().class("col-md-6"),
                ))
                .into()
        })
        .into()
    }
}

#[derive(Clone)]
pub enum Message {
    Create(u32),
    Append(u32),
    UpdateEvery(u32),
    Clear,
    Swap,
    Remove(u32),
    Select(u32),
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

    fn buttons() -> impl Iterator<Item = draco::VNode<Message>> {
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
                .with(
                    h::button()
                        .id(button.id)
                        .class("btn btn-primary btn-block")
                        .type_("button")
                        .on("click", move |_| button.message.clone())
                        .with(button.description),
                )
                .into()
        })
    }
}

impl draco::Application for Jfb {
    type Message = Message;

    fn update(&mut self, message: Self::Message, mailbox: &draco::Mailbox<Self::Message>) {
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
                mailbox.send(Message::Append(amount));
            }
            Message::Append(amount) => {
                rows.extend((0..amount).map(|index| Row::new(*next_id + index, rng)));
                *next_id += amount;
            }
            Message::UpdateEvery(step) => {
                for index in (0..rows.len()).step_by(step as usize) {
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
                rows.retain(|row| row.id != id);
            }
            Message::Select(id) => {
                if *selected_id == Some(id) {
                    *selected_id = None;
                } else {
                    *selected_id = Some(id);
                }
            }
        }
    }

    fn view(&self) -> draco::VNode<Message> {
        use draco::html as h;

        h::div()
            .class("container")
            .with((
                h::div()
                    .class("jumbotron")
                    .with(h::div().class("row").with((
                        h::div().class("col-md-6").with(h::h1().with("Draco")),
                        h::div().class("col-md-6").append(Self::buttons()),
                    ))),
                h::table()
                    .class("table table-hover table-striped test-data")
                    .with({
                        let vnode: draco::VNode<Message> = if self.keyed {
                            draco::html::keyed::tbody()
                                .id("tbody")
                                .append(self.rows.iter().map(|row| {
                                    (row.id as u64, row.view(self.selected_id == Some(row.id)))
                                }))
                                .into()
                        } else {
                            h::tbody()
                                .id("tbody")
                                .append(
                                    self.rows
                                        .iter()
                                        .map(|row| row.view(self.selected_id == Some(row.id))),
                                )
                                .into()
                        };
                        vnode
                    }),
                h::span()
                    .class("preloadicon glyphicon glyphicon-remove")
                    .attribute("aria-hidden", "true"),
            ))
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
