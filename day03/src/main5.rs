fn main() {
    test1();
    test2();
}

fn test2() {
    let s = String::new();
    let update_string = move || println!("{}", s);
    exec(update_string);
    // fn exec<F: FnOnce()>(f: F) {
    //     f()
    // }

    fn exec<F: Fn()>(f: F) {
        f()
    }
}

fn test1() {
    let s = "hello".to_string();
    let update_string = |str| println!("{} {}", s, str);
    exec(update_string);
    println!("{:?}", s);
    fn exec<F: Fn(String) -> ()>(f: F) {
        f("world".to_string());
    }
}
struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cache<T> {
        Cache { query, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
