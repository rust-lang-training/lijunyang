pub fn test() {
    let x = 10;
    let c1 = || {
        println!("{}", x);
    };

    c1();
    c1();
    // Fn  可以被多次调用 这种闭包不能捕获变量的值， 可以使用 & 来捕获变量的引用， 对于copy类型 则默认会以不可变引用的方式来捕获他
    println!("{}", x);

    let mut y = 10;
    let mut c2 = || {
        y += 10;
    };
    // FnMut 可以被多次调用， 这种闭包可以改变捕获变量的值
    c2();
    c2();
    println!("{}", y);

    let z = "rust".to_string();
    // FnOnce 只能被调用一次 这种闭包调用后会将 捕获的变量的所有权移动到闭包内部， 可以使用move关键字来捕获变量的所有权。
    // 因为闭包会移动变量的所有权，所以在使用完后就不能再次访问这些变量了
    let c3 = move || {
        println!("{}", z);
    };

    c3();
    // println!("{}", z);

    {
        let x = 5;
        let c1 = || {
            println!("{}", x);
        };

        let mut y = 6;
        let mut c2 = || {
            y += 1;
            println!("{}", y);
        };

        let z = "rust".to_string();
        let c3 = move || {
            println!("{}", z);
        };

        fn is_Fn<F>(_: &F)
        where
            F: Fn() -> (),
        {
        }

        fn is_FnMut<F>(_: &F)
        where
            F: FnMut() -> (),
        {
        }

        fn is_FnOnce<F>(_: &F)
        where
            F: FnOnce() -> (),
        {
        }

        is_Fn(&c1);
        is_FnMut(&c1);
        is_FnOnce(&c1);

        is_FnMut(&c2);
        is_FnOnce(&c2);

        is_FnOnce(&c3);

        // is_FnMut(&c3);
        // is_Fn(&c2);

        // 通常一个闭包可以被建模为一个结构体
        // struct Closure<'l0...'li, T0...Tj, CK, CS, U>(...U)
        // 'l0...'li 是生命周期参数
        // T0...Tj 是 泛型参数
        // CK 是闭包的类型 ClosureKind 的缩写， 有 Fn FnMut FnOnce 三种
        // pub enum ClosureKind { Fn, FnMut, FnOnce }
        // CS 表示闭包的签名 是 Closure Signatures的缩写 类似于函数的签名， 看作是fn() 类型， 例如 fn(u32, u32) -> u32 意味着闭包实现了
        // CK<(u32, u32), Output = u32>
        // U 可访问的参数的类型，
    }
    {
        trait Animal {
            fn make_sound(&self);
        }

        struct Dog {
            name: String,
        }

        struct Cat {
            name: String,
        }

        impl Animal for Dog {
            fn make_sound(&self) {
                println!("wangwang");
            }
        }
        impl Animal for Cat {
            fn make_sound(&self) {
                println!("miaomiao");
            }
        }

        let dog = Dog {
            name: "二哈".to_string(),
        };
        let cat = Cat {
            name: "美短".to_string(),
        };
        dog.make_sound();
        cat.make_sound();

        fn speak(animal: impl Animal) {
            animal.make_sound();
        }
        impl Animal for i32 {
            fn make_sound(&self) {
                println!("i32");
            }
        }

        trait Test {
            fn test(&self);
        }

        impl Test for Dog {
            fn test(&self) {
                println!("这是一个测试方法");
            }
        }

        speak(dog);
        speak(cat);
        speak(8);
        let dog = Dog {
            name: "二哈".to_string(),
        };
        fn printMulti(p: impl Test + Animal) {
            p.make_sound();
            p.test();
        }

        printMulti(dog);
    }
    {
        trait MyPrint<T> {
            fn print(&self, x: T) -> T;
        }
        struct Test;

        impl MyPrint<i32> for Test {
            fn print(&self, x: i32) -> i32 {
                x
            }
        }
        let test = Test;
        test.print(8);

        // fn trait_demo<T: TraitOne + TraitTwo + TraitOther>(param: T)
        // fn trait_fun<T: TraitOne, E: TraitTwo + TraitOther>(param1: T, param2: E)
        // fn multi_fun_where<T, E>(param1: T, param2: E) where T: TraitOne, E: TraitTwo + TraitOther {}
    }

    {
        // rust中没有继承的概念 可以定一个trait为另一个trait的超集
        trait Animal {
            fn speak(&self);
        }
        trait Dog: Animal {
            fn jump(&self);
        }

        struct SmallDog;
        impl Animal for SmallDog {
            fn speak(&self) {}
        }
        impl Dog for SmallDog {
            fn jump(&self) {}
        }
    }
}
