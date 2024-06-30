fn main() {
    println!("Hello, world!");
    let s = "hello world";
    let s1 = &s[0..5];
    let s2 = &s[6..s.len()];
    println!("{} and {}", s1, s2);
}
