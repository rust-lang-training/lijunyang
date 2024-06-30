fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let names = [
        String::from("John"),
        String::from("Tom"),
        String::from("Penny"),
        String::from("Sheldon"),
    ];

    for i in 0..4 {
        // 使用下标访问数组中的值， 也会发生所有权转移
        // let s = names[i];
        // println!("{}", s);
    }
    println!("names[0] = {}", names[0]);

    let s = String::from("hello World");
    //  moved due to this method call
    let bytes = s.into_bytes(); // pub fn into_bytes(self) -> Vec<u8
                                // println!("{:?}", s);

    let x = 10;
    let y = &x;
    assert!(x == *y); // *y 是对y进行解引用， 获取y所指向的值
    assert!(y == &10);
    println!("{}", y == &10);

    let mut x = 10;
    let y = &mut x;
    *y = 20; // 简单类型需要自行解引用， a.x 点号操作 会自动解引用
             // x = 40;
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s2;
    let s4 = &s3;
    let s5 = &s4;
    let len = calculate_length(&&&&&&s5);
    println!("The length of '{}' is {}.", s1, len);
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    // 引用规则  在任意时刻 只能有一个可变引用 多个不可变引用

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let s1 = String::from("xx");
    let s2 = String::from("😊");
    let res = longest(&s1, &s2);
    println!("{}", res);

    let mut s = String::from("hello world");
    let rs = &s;

    // s.push_str(" I am rust"); // fn push_str(&mut self, string: &str)
    println!("The string is {}", rs);

    let s1 = String::from("hello");
    let rs1 = &s1;
    // let s2 = s1;
    println!("{}", rs1);

    // let bytes = gen_string().as_bytes(); // temporary value dropped while borrowed
    // let s = &gen_string();
    let s = gen_string();
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    fn gen_string() -> String {
        String::from("hello world")
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

/**
 * 每个值都有一个所有者 Each value in Rust has an owner
 * 在同一时刻，一个只知能有一个所有者 There can only bi one owner at a time
 * 当值得拥有者离开作用域，值将被丢弃，所占用的内存也被释放 When the owner goes out of scope, the value will be dropped
 */
fn a() {}
