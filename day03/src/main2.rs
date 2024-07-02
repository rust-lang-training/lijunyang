fn main() {
    test1();
    test2();
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
