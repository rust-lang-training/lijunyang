fn main() {
    println!("Hello, world!");
}

// std::result::Result
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub trait Error: Debug + Display {
    fn source(&)
}