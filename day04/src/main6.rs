fn main() {}

struct Book {
    name: String,
    price: usize,
}
// #[derive(PartialEq)] rust 会逐字段比较  如果是enum 对于enum中的数据进行比较
// 自定义 PartialEq
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// 实现Eq的前提是 已经实现 PartialEq  如果已经实现了 PartialEq 可以直接使用 #[derive(Eq)] 如果struct或者 enum中存在浮点数时， 需要手动实现改trait

impl Eq for Book {}

// Ord PartialOrd
// Ord PartialOrd 均可通过#[derive] 派生宏交由编译器自动实现 当使用派生宏宏 将会基于struct的字段以字典序进行比较， 遇到枚举中的数据也会以此类推，
// 注意实现 PartialOrd要求该类型实现 PartialEq

struct Person {
    name: String,
    age: usize,
}
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}
impl Eq for Person {}
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.age.partial_cmp(&other.age)
    }
}
// Ord 要求你的类型实现 PartialOrd 和 Eq
impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.age.cmp(&other.age)
    }
}

struct Dog {
    weight: usize,
    height: usize,
}

struct People {
    iq: isize,
    eq: isize,
}

impl From<Dog> for People {
    fn from(value: Dog) -> Self {
        People {
            iq: value.weight as isize + 10,
            eq: -1 * value.height as isize,
        }
    }
}
// impl Into<People> for Dog {
//     fn into(self) -> People {
//         People {
//             iq: self.weight as isize + 10,
//             eq: -1 * self.height as isize,
//         }
//     }
// }
fn test() {
    let d1 = Dog {
        weight: 20,
        height: 40,
    };
    let d2 = Dog {
        height: 30,
        weight: 30,
    };

    let p1 = People::from(d1);
    let p2: People = d2.into();
}

// impl TryFrom<Dog> for People {
//     type Error = String;

//     fn try_from(value: Dog) -> Result<Self, Self::Error> {
//         if value.weight < 0 as usize || value.height < 0 as usize {
//             Err("转换出错 => Dog to People".to_string())
//         } else {
//             Ok(People {
//                 iq: value.weight as isize,
//                 eq: value.height as isize,
//             })
//         }
//     }
// }
