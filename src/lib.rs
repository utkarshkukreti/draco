#[macro_use]
pub mod console;
mod application;
mod aspect;
mod attribute;
pub mod html;
mod lazy;
mod listener;
mod mailbox;
mod property;
pub mod router;
pub mod subscription;
pub mod svg;
pub mod url;
mod velement;
mod vnode;
mod vtext;

pub use self::application::{start, Application};
pub use self::aspect::Aspect;
pub use self::attribute::Attribute;
pub use self::lazy::Lazy;
pub use self::listener::Listener;
pub use self::mailbox::Mailbox;
pub use self::property::Property;
pub use self::subscription::{Subscription, Unsubscribe};
pub use self::velement::{h, s};
pub use self::velement::{VElement, VKeyedElement, VNonKeyedElement};
pub use self::vnode::VNode;
pub use self::vtext::VText;

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
