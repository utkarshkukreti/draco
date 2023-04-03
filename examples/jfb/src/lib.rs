use wasm_bindgen::prelude::*;
use web_sys as web;
use xorshift::Xorshift128;

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
    rng: Xorshift128,
    keyed: bool,
}

#[derive(Clone, Hash)]
struct Row {
    id: u32,
    label: String,
}

impl Row {
    fn new<R: xorshift::Rng>(id: u32, rng: &mut R) -> Row {
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
            rng: xorshift::SeedableRng::from_seed([0].as_slice()),
            keyed,
        }
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

        let button = |id, description, message: Message| -> draco::VNode<Message> {
            h::div()
                .class("col-sm-6 smallpad")
                .with(
                    h::button()
                        .id(id)
                        .class("btn btn-primary btn-block")
                        .type_("button")
                        .on("click", move |_| message.clone())
                        .with(description),
                )
                .into()
        };

        h::div()
            .class("container")
            .with((
                h::div()
                    .class("jumbotron")
                    .with(h::div().class("row").with((
                        h::div().class("col-md-6").with(h::h1().with("Draco")),
                        h::div().class("col-md-6").with((
                            button("run", "Create 1,000 rows", Message::Create(1000)),
                            button("runlots", "Create 10,000 rows", Message::Create(10000)),
                            button("add", "Append 1,000 rows", Message::Append(1000)),
                            button("update", "Update every 10th row", Message::UpdateEvery(10)),
                            button("clear", "Clear", Message::Clear),
                            button("swaprows", "Swap Rows", Message::Swap),
                        )),
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
