use draco::{Lazy, Mailbox, VNode};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_test::*;
use web_sys as web;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn t_same_arg_but_different_function() {
    let mailbox = Mailbox::new(|_| ());
    let mut node_1: VNode<()> = Lazy::new(1, |&x| x.to_string().into()).into();
    let web_node = node_1.create(&mailbox);
    assert_eq!(
        web_node
            .dyn_into::<web::Text>()
            .unwrap_throw()
            .text_content()
            .unwrap_throw(),
        "1"
    );
    let mut node_2: VNode<()> = Lazy::new(1, |&x| (x + 1).to_string().into()).into();
    let web_node = node_2.patch(&mut node_1, &mailbox);
    assert_eq!(
        web_node
            .dyn_into::<web::Text>()
            .unwrap_throw()
            .text_content()
            .unwrap_throw(),
        "2"
    );
}
