fn main() {
    test1();
    test2();
    let s = "hello world";
    let s1 = &s[0..5];
    let s2 = &s[6..s.len()];
    println!("{} {}", s1, s2);
}

fn test1() {
    println!("{}", describe_point(1, 0));
    println!("{}", describe_point(0, 1));
    println!("{}", describe_point(1, 1));
    println!("{}", describe_point(-1, 1));
    println!("{}", describe_point(-1, -1));
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;

        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "at the origin",
            (_, Equal) => "a the x axis",
            (Equal, _) => "at the y axis",
            (Greater, Greater) => "in the first quadrant",
            (Less, Greater) => "in the second quadrant",
            _ => "somewhere else",
        }
    }

    println!("{}", fabnicco(1));
    println!("{}", fabnicco(2));
    println!("{}", fabnicco(3));
    println!("{}", fabnicco(4));
    println!("{}", fabnicco(5));
    println!("{}", fabnicco(6));
    println!("{}", fabnicco(7));
    println!("{}", fabnicco(8));
}

fn fabnicco(n: u64) -> u64 {
    match n {
        // 1 => 1,
        // 2 => 1,
        n @ 1..=2 => n,
        _ => fabnicco(n - 1) + fabnicco(n - 2),
    }
}

fn test2() {
    let s = "alkasd5729^&@fg";

    let mut iter = s.chars();

    loop {
        match iter.next() {
            Some(n @ '0'..='9') => println!("{} 是 数字类型", n),
            Some(n @ ('a'..='z' | 'A'..='Z')) => println!("{} 是字母类型", n),
            Some(c) => println!("{} 是 特殊字符类型", c),
            None => {
                println!("over");
                break;
            }
        }
    }
}
