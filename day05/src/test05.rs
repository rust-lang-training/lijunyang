use std::collections::HashMap;
pub fn test() {
    let vec = vec![1, 2, 3, 4, 5];
    // filter_map() 是filter 和 map的组合
    // 接收一个闭包类型，返回一个Option类型 如果闭包返回None，就表示从迭代器中过滤掉这个元素，如果返回Some(x), 表示保留x值
    let x: Vec<_> = vec
        .iter()
        .filter_map(|item| if (item % 2 == 0) { None } else { Some(item) })
        .collect();

    println!("{:?}", x);

    let mut map = HashMap::new();
    map.insert("animals", vec!["dog", "cat", "pig"]);
    map.insert("fruits", vec!["apple", "banana", "orange"]);
    map.insert("cities", vec!["beijing", "shanghai", "guangzhou"]);
    let vec = vec!["animals", "fruits", "cities"];

    let result: Vec<&Vec<&str>> = vec.into_iter().flat_map(|item| map.get(item)).collect();
    println!("{:?}", result);

    // scan 接收一个初始状态和闭包， 闭包又可以接受一个对初始值状态的可修改引用 和 迭代项， 闭包的返回值是Option是，如果闭包反回None则将终止迭代

    let vec = vec![1, 2, 3, 4, 5];
    let result: Vec<_> = vec
        .iter()
        .scan(0, |init, item| {
            let x = item * item;
            *init += x;
            if *init > 50 {
                None
            } else {
                Some(x)
            }
        })
        .collect();
    println!("{:?}", result);
}
