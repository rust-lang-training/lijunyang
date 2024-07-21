pub fn test() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for i in vec {
        println!("{}", i);
    }

    let mut vec = vec![1, 2, 3];
    let mut iter = vec.into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }

    struct Example(i32);

    impl Iterator for Example {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 % 10 != 0 {
                self.0 += 1;
                Some(self.0)
            } else {
                self.0 += 1;
                None
            }
        }
    }

    let mut example = Example(8);

    // for item in example.into_iter() {
    //     println!("{}", item);
    // }

    println!("为熔断== {:?}", example.next());
    println!("为熔断== {:?}", example.next());
    println!("为熔断== {:?}", example.next());
    println!("为熔断== {:?}", example.next());
    println!("为熔断== {:?}", example.next());
    println!("为熔断== {:?}", example.next());
    println!("为熔断== {:?}", example.next());
    // 大部分的迭代器 都不会出现None， Some交替返回的现象，
    // 为防止意外发生， Rust提供了熔断的方法，一旦迭代的值出现 None， 后续不会再次出现Some
    let mut example = Example(8).fuse(); // 在Example 后调用fuse方法 ，可以触发熔断机制，
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
    println!("熔断后== {:?}", example.next());
}
