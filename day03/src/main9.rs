use std::rc::Rc;
fn main() {
    let s = Rc::new(String::from("hello"));
    println!("ref count: {}", Rc::strong_count(&s));
    {
        let t = s.clone();
        println!("ref count: {}", Rc::strong_count(&t));
    }
    let u = s.clone();

    println!("ref count: {}", Rc::strong_count(&s));
    println!("ref count: {}", Rc::strong_count(&u));

    let b = Box::new(5);
    println!("{}", b);
    test1();
}
use std::cell::RefCell;
fn test1() {
    let s = RefCell::new(String::from("hello"));
    append_string(&s);
    println!("{}", s.borrow());
    fn append_string(s: &RefCell<String>) {
        let rs = s.borrow();
        let mut ms = s.borrow_mut();
        (*ms).push_str(" world!");
        println!("{}", rs);
    }
}
