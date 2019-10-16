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
        jfb.update(&mailbox, jfb::Message::Create(1000));
    });
}

#[bench]
fn bench_render_1000(b: &mut test::Bencher) {
    let mut jfb = jfb::Jfb::new(true);
    let mailbox = draco::Mailbox::new(|_| {});
    jfb.update(&mailbox, jfb::Message::Create(1000));
    b.iter(|| {
        jfb.view();
    });
}
