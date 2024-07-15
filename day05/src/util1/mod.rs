pub fn hello2() {
    println!("Hello2");
}

#[derive(Debug)]
pub enum TestEnum {
    A,
    B,
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: f32,
}
