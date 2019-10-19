#[macro_use]
pub mod console;
pub mod app;
pub mod aspect;
pub mod attribute;
pub mod element;
pub mod html;
pub mod listener;
pub mod mailbox;
pub mod node;
pub mod property;
pub mod router;
pub mod subscription;
pub mod svg;
pub mod text;
pub mod url;

pub use self::app::{start, Application};
pub use self::aspect::Aspect;
pub use self::attribute::Attribute;
pub use self::element::{h, s};
pub use self::element::{Element, KeyedElement, NonKeyedElement};
pub use self::listener::Listener;
pub use self::mailbox::Mailbox;
pub use self::node::Node;
pub use self::property::Property;
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
