pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T> {
    fn into(self) -> T;
}
fn main() {
    let p = <Person as From<&str>>::from("Alice,30");
    println!("{p:?}");

    println!("{}", <Person as Into<String>>::into(p));

    let p1 = Person {
        name: "Bob".to_owned(),
        age: 35,
    };

    let p_str = <Person as Into<String>>::into(p1);
    println!("{}", p_str);

    let s1 = "hello";
    let s2: String = s1.into();

    println!("{}", s2);

    {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl std::convert::From<i32> for Number {
            fn from(value: i32) -> Self {
                Number { value }
            }
        }

        let num = Number::from(32);

        println!("{:?}", num);

        let int = 5;

        let num: Number = int.into();
        println!("{}", num.value);
        println!("{:?}", int);
    }

    {
        let num = 5;
        let s: String = num.to_string();
        println!("{}", s)
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl From<&str> for Person {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(",").collect();
        let name = parts[0].to_string();
        let age: u32 = parts[1].parse().unwrap();
        Person { name, age }
    }
}

impl Into<String> for Person {
    fn into(self) -> String {
        format!("{} ({} years old)", self.name, self.age)
    }
}
