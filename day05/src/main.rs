mod copy;
mod util;
mod util1;
use util1::{hello2, Point, TestEnum};
mod async1;
mod hw;
mod log;
mod mutex;
mod pi;
mod test;
mod test02;
mod test03;
mod test04;
// use async1::test;
use futures::future::join_all;
use std::time;
#[async_std::main]
async fn main() {
    println!("Hello, world!");
    util::hello();
    hello2();
    // util1::hello();
    let a = Point { x: 1, y: 2.0 };
    println!("{:?}", a);
    let b = TestEnum::A;
    println!("{:?}", b);

    // copy::copy_file("file.txt", "test.txt");
    // copy::copy_file(
    //     "/Users/liyaya/work/project/maler/dist/js/async-chunk-vendors.46532fd2.js",
    //     "1.js",
    // );
    // copy::copy_file(
    //     "/Users/liyaya/Downloads/mysql-8.0.36-macos14-arm64.tar.gz",
    //     "1.tar.gz",
    // );
    // test::copy_file("file.txt", "test.txt").unwrap();

    // log::test();

    // mutex::test();

    // pi::test();
    // async1::test();
    // async1::test()
    // async1::test().await;
    // test02::test02_main().await;
    // test03::test03_main().await;

    // test04::test_main();

    hw::test();
}
// std::future::Future
// futures::future::Future
// Streams Read Write Seek BufRead traits
