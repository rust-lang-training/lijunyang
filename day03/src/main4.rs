fn main() {
    let day = 5;
    match day {
        0 | 6 => println!("weekend"),
        1..=5 => println!("weekday"),
        _ => println!("invalid day"), // _ is a catch-all pattern
    }

    let c = 'w';
    match c {
        'a'..='z' => println!("小写字母"),
        'A'..='Z' => println!("大写字母"),
        _ => println!("其他字符"),
    }

    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;

    match dire {
        Direction::East => println!("东"),
        Direction::West => println!("西"),
        Direction::North => println!("北"),
        Direction::South => println!("南"),
    }

    let x = 1;
    match x {
        n @ 1..=5 => println!("{}", n),
        _ => println!("other"),
    }

    let x = String::from("下雨了");
    // 注意模式匹配中的所有权转移
    match x {
        // r => println!("{}", r),
        ref r => println!("Got a reference to {}", r),
    }
    println!("{}", x);

    let pair = (0, -2);

    match pair {
        (0, y) => println!("x is zero and y is {}", y),
        (x, 0) => println!("x is {} and y is zero", x),
        _ => println!("x and y are not 0"),
    }

    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }
    // 解构枚举
    // let action = Action::Say("Hello Rust".to_string());
    let action = Action::MoveTo(1, 1);
    match action {
        Action::Say(s) => println!("{}", s),
        Action::MoveTo(x, y) if x < 2 && y < 2 => println!("x 小于 2 and y 小于 2"),
        Action::MoveTo(x, y) => {
            println!("point from (0, 0) move to ({}, {})", x, y);
        }
        Action::ChangeColorRGB(r, g, _) => {
            println!("change color to RGB({}, {}, 255)", r, g);
        }
    }

    // 解构结构体
    struct Point {
        x: i64,
        y: i64,
    }
    let p = Point { x: 0, y: 1 };
    match p {
        Point { x, y } => println!("({}, {})", x, y),
    }
    match p {
        Point { x: x1, y: y1 } => println!("x1 is {} and y1 is {}", x1, y1),
    }
    match p {
        Point { y, .. } => {
            println!("y is {}", y);
        }
    }

    // match 守卫

    let x = 10;
    let y = false;

    match x {
        4 | 5 if y => println!("x is 4 or 5 and y is true"),
        n @ 8..=10 if n % 2 == 0 => println!("x is {}, x 是偶数", n),
        _ => println!("other"),
    }
}
