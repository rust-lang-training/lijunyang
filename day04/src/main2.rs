use std::{fmt::Debug, io, ops::Add, path::Path, result};

fn main() {
    let p = Point { x: 1.0, y: 2.0 };

    println!("{:?}", p);

    check_file("/a/b/c/d");
    check_file("/a/b/c/d".to_string());

    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };

    println!("Area: {}", rect.area());
    println!("Area: {}", rect.perimeter());
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// thiserror crate 处理Error
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn check_file<P: AsRef<Path>>(_path: P) -> result::Result<(), io::Error> {
    let a = _path.as_ref();
    println!("{:?}", a);
    Ok(())
}

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<
        T: std::ops::Mul<T, Output = T> + Copy + Add<T, Output = T> + std::convert::From<i32> + Debug,
    > Rectangle<T>
{
    fn area(&self) -> T {
        self.width * self.height
    }

    fn perimeter(&self) -> T {
        // <i32 as Into<T>>::into(2) * (self.width + self.height)
        // <i32 as Into<T>>::into(2) * (self.width + self.height)
        let a = <i32 as Into<T>>::into(2);
        println!("{:?}", a);
        a * (self.width + self.height)
    }
}
