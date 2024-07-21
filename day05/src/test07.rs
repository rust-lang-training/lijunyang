pub fn test() {
    let mut vec = vec!["hello", "my", "name", "is", "rust", "!"];
    let mut my_iter = vec.iter_mut();

    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());

    for item in vec.iter() {
        println!("{:?}", item);
    }

    // 当集合的引用调用 into_iter() 方法时
    // (&vec).into_iter() // 不可变引用， into_iter会产生迭代项的 共享引用的迭代器
    // (&mut vec).into_iter() // 可变引用 into_iter 会产生可修改引用的迭代器
    let vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut iter = (&vec).into_iter();
    println!("{:?}", iter.next());

    let mut vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut iter = (&mut vec).into_iter();
    iter.next().unwrap().push_str(" hello");
    println!("{:?}", vec);

    // for item in &vec => (&vec).into_iter()
    // for item in &mut vec => (&mut vec).into_iter();
    // for item in vec => mut vec.into_iter();
}
