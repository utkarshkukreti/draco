#[macro_use]
pub mod console;
pub mod app;
pub mod element;
pub mod fetch;
pub mod html;
pub mod mailbox;
pub mod node;
pub mod router;
pub mod subscription;
pub mod svg;
pub mod text;

pub use self::app::{start, App, Instance};
pub use self::element::{h, s};
pub use self::element::{Element, KeyedElement, NonKeyedElement};
pub use self::mailbox::Mailbox;
pub use self::node::Node;
pub use self::subscription::{Subscription, Unsubscribe};
pub use self::text::Text;
use std::borrow::Cow;

pub type S = Cow<'static, str>;

pub fn select(selector: &str) -> Option<web_sys::Element> {
    web_sys::window()?
        .document()?
        .query_selector(selector)
        .ok()?
}
