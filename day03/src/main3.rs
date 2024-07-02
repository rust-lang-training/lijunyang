fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}

fn test6() {
    let t1 = (5, 5);
    let (x, y) = t1;
    println!("{} {} {:?}", x, y, t1);

    let t2 = (5, String::from("hello"));
    let (x, ref y) = t2;

    println!("{} {} {:?}", x, y, t2);
    let (x, _) = t2;
    println!("{}", x);
}

fn test5() {
    use std::ops::Deref;
    use std::ops::DerefMut;
    struct DerefExample<T> {
        value: T,
    }
    impl<T> Deref for DerefExample<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    let x = DerefExample { value: 100 };
    println!("{}", *x);

    struct DerefMutExample<T> {
        value: T,
    }
    impl<T> Deref for DerefMutExample<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl<T> DerefMut for DerefMutExample<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    let mut x = DerefMutExample { value: 'a' };
    *x = 'b';
    println!("{}", *x);
}
fn test4() {
    enum Person {
        Male,
        Female,
    }

    impl Person {
        fn name(&self) -> String {
            match *self {
                Person::Male => String::from("he is a man"),
                Person::Female => String::from("he is a woman"),
                _ => String::from("it is not human"),
            }
        }
        fn apply(&self) {
            println!("xxx");
        }
    }
    let p = Person::Male;
    p.name();
    p.apply();
}

fn test3() {
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    impl Message {
        fn call(&self) {
            match self {
                &Message::Move { x, y } => println!("Move : {}, {}", x, y),
                &Message::Quit => println!("Quit"),
                &Message::ChangeColor(a, b, c) => println!("ChangeColor: {}, {}, {}", a, b, c),
                &Message::Write(ref s) => println!("Write: {}", s),
            }
        }
    }

    let msg = Message::Move { x: 1, y: 2 };
    msg.call();
}

fn test2() {
    #[derive(Debug)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut p = Point3d { x: 1, y: 2, z: 3 };
    let p1 = &mut p;
    p1.x = 10;
    *p1 = Point3d {
        x: 100,
        y: 200,
        z: 300,
    };
    println!("{:?}", p);

    {
        let p2 = &mut p;
        *p2 = Point3d { x: 3, y: 2, z: 1 };
    }
    println!("{:?}", p);
}

fn test1() {
    struct Person {
        name: String,
    }

    impl Person {
        fn new(n: &str) -> Person {
            Person {
                name: n.to_string(),
            }
        }
        fn greeting(&self) {
            println!("{} say hello", self.name);
        }
    }

    let peter = Person::new("Peter");
    peter.greeting();
}
