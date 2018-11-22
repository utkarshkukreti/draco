#[macro_use]
pub mod console;
pub mod app;
pub mod attr;
pub mod element;
pub mod html;
pub mod mailbox;
pub mod node;
pub mod observe;
pub mod router;
pub mod subscription;
pub mod svg;
pub mod text;
pub mod url;

pub use self::app::{start, App, Instance};
pub use self::attr::Attr;
pub use self::element::{h, s};
pub use self::element::{Element, KeyedElement, NonKeyedElement};
pub use self::mailbox::Mailbox;
pub use self::node::Node;
pub use self::observe::Observe;
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

pub fn set_panic_hook() {
    use std::sync::Once;

    static PANIC_HOOK: Once = Once::new();

    PANIC_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|panic| {
            crate::console::error(&panic.to_string());
        }));
    });
}
