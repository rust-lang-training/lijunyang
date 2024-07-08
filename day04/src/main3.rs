fn main() {
    let x = 42;

    let c1 = || println!("hello world");
    let c2 = || println!("{}", x);

    let c3 = || {
        let s = String::from("hello world");
        println!("{}", s);
    };

    c1();
    c2();
    c3();
    {
        let a = 100;
        {
            let b = 200;
            let c = || {
                println!("{a}");
                println!("{b}");
            };
            c();
        }
    }

    let mut s = String::from("hello world");
    let mut c1 = || {
        s.push_str(" world");
    };

    c1();
    println!("{}", s);

    {
        let s = String::from("hello world");
        let closure = move || {
            let tp = (s, 1);
            println!("{:?}", tp);
        };

        closure();
        // closure();
    }

    {
        let add_one = |x: i32| x + 1;
        let y = add_one(10);
        println!("y = {y}");
    }
}
