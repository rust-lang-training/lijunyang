use std::rc::Rc;

pub fn test() {
    let a = Rc::new(String::from("hello"));
    let b = a.clone();
    let c = Rc::clone(&a);
    // Rc::clone(&a) 等价于 a.clone()
    println!("{:p}", a.as_ptr());
    println!("{:p}", b.as_ptr());
    println!("{:p}", c.as_ptr());
    println!("{}", Rc::strong_count(&a));
    println!("{}", Rc::weak_count(&a));
    println!("{}", a.contains("he"));
}
