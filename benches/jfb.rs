#![feature(test)]

extern crate test;

#[path = "../examples/jfb/src/lib.rs"]
#[allow(dead_code)]
mod jfb;

use draco::App;

#[bench]
fn bench_create_1000(b: &mut test::Bencher) {
    b.iter(|| {
        let mut jfb = jfb::Jfb::new(true);
        let mailbox = draco::Mailbox::new(|_| {});
        jfb.update(jfb::Message::Append(1000), &mailbox);
    });
}

#[bench]
fn bench_render_1000(b: &mut test::Bencher) {
    let mut jfb = jfb::Jfb::new(true);
    let mailbox = draco::Mailbox::new(|_| {});
    jfb.update(jfb::Message::Append(1000), &mailbox);
    b.iter(|| {
        jfb.view();
    });
}
