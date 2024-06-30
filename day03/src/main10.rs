fn main() {
    let user = User {
        active: false,
        username: String::from("lucy"),
        email: String::from("lucy@gmail.com"),
        sing_in_count: 1,
    };

    println!("{:?}", user);
    println!("{}", user.sing_in_count);
    let User {
        active, username, ..
    } = user;
    println!("active is {}", active);
    println!("username is {}", username);

    let mut user2 = User {
        username: String::from("lily"),
        email: String::from("lily@gmail.com"),
        ..user
    };
    // 结构体的可变性控制到每个属性
    user2.sing_in_count = 3;
    println!("{}", user2.sing_in_count);
    println!("{:?}", user2);

    struct Point(f32, f32);

    let p = Point(3.0, 4.0);

    println!("{} {}", p.0, p.1);

    let Point(x, y) = p;
    println!("{} {}", x, y);

    #[derive(Debug)]
    struct AlwaysEqual; // 单元式结构体
    let subject = AlwaysEqual;
    println!("{:?}", subject);

    test2();
    // rust中的结构体不支持继承
    // 组合优先继承

    test3();
}
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

fn test3() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let q = Message::Quit;
    let m = Message::Move { x: 100, y: 200 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(255, 255, 255);
    println!("{:?}", q);
    println!("{:?}", m);
    println!("{:?}", w);
    println!("{:?}", c);
}

fn test2() {
    let mut r1 = Rectangle::new(2.0, 3.0);
    println!("area is {}", r1.area());
    println!("perimeter is {}", r1.perimeter());
    r1.scale(2.0, 2.0);
    println!("{:?}", r1);

    let s2 = Rectangle::square(5.0);
    println!("{:?}", s2);
    #[derive(Debug)]
    struct Rectangle {
        width: f32,
        height: f32,
    }

    impl Rectangle {
        fn square(size: f32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }

        fn new(width: f32, height: f32) -> Rectangle {
            Rectangle { width, height }
        }

        fn perimeter(&self) -> f32 {
            (self.width + self.height) * 2.0f32
        }

        fn area(&self) -> f32 {
            self.width * self.height
        }

        fn scale(&mut self, width_scale: f32, height_scale: f32) {
            self.width *= width_scale;
            self.height *= height_scale;
        }
    }
}
