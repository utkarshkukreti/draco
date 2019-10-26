use wasm_bindgen::prelude::*;

struct GitHubCommits {
    repo: String,
    response: Option<Response>,
}

type Response = Result<Vec<Record>, Box<dyn std::error::Error>>;

#[derive(serde::Deserialize)]
struct Record {
    html_url: String,
    sha: String,
    commit: Commit,
}

#[derive(serde::Deserialize)]
struct Commit {
    author: Author,
    message: String,
}

#[derive(serde::Deserialize)]
struct Author {
    name: String,
    date: String,
}

impl GitHubCommits {
    fn new() -> Self {
        GitHubCommits {
            repo: "rust-lang/rust".into(),
            response: None,
        }
    }
}

enum Message {
    Fetch,
    UpdateRepo(String),
    UpdateResponse(Response),
}

impl draco::Application for GitHubCommits {
    type Message = Message;

    fn update(&mut self, message: Self::Message, mailbox: &draco::Mailbox<Self::Message>) {
        use self::Message::*;
        match message {
            Fetch => {
                let url = format!(
                    "https://api.github.com/repos/{}/commits?per_page=10",
                    self.repo
                );
                match url.parse::<reqwest::Url>() {
                    Ok(url) => mailbox.spawn(
                        // TODO: s/text()/json()/ and remove serde_json when it's implemented for WASM
                        async {
                            Ok(serde_json::from_str(
                                &reqwest::get(url).await?.text().await?,
                            )?)
                        },
                        Message::UpdateResponse,
                    ),
                    Err(err) => draco::console::error(&err.to_string()),
                }
            }
            UpdateRepo(repo) => self.repo = repo,
            UpdateResponse(response) => self.response = Some(response),
        }
    }

    fn view(&self) -> draco::VNode<Self::Message> {
        use draco::html as h;
        h::form()
            .on("submit", |event| {
                event.prevent_default();
                Message::Fetch
            })
            .push(
                h::input()
                    .value(self.repo.clone())
                    .on_input(Message::UpdateRepo),
            )
            .push(h::button().push("Fetch").on("click", |_| Message::Fetch))
            .push(match &self.response {
                Some(Ok(records)) => h::ul().append(records.iter().map(|record| {
                    h::li().push(
                        h::p()
                            .push(
                                h::a()
                                    .href(record.html_url.clone())
                                    .push(record.sha[0..8].to_string()),
                            )
                            .push(" ")
                            .push(
                                record
                                    .commit
                                    .message
                                    .lines()
                                    .next()
                                    .unwrap_or("")
                                    .to_string(),
                            )
                            .push(h::br())
                            .push("By ")
                            .push(h::strong().push(record.commit.author.name.clone()))
                            .push(" at ")
                            .push(record.commit.author.date.clone()),
                    )
                })),
                Some(Err(err)) => h::pre().push(format!("{:#?}", err)),
                None => h::div(),
            })
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        GitHubCommits::new(),
        draco::select("main").expect("<main>").into(),
    );
}
