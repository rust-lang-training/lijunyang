fn main() {
    println!("Hello, world!");
    let s = "hello world";
    let s1 = &s[0..5];
    let s2 = &s[6..s.len()];
    println!("{} and {}", s1, s2);

    #[derive(Debug, Default)]
    // 结构体添加 Default特征 结构体会自动实现 Default 特征
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point3d { x: 1, y: 2, z: 3 };
    println!("{:?}", p);
    println!("{:#?}", p);

    let p2 = Point3d::default();
    println!("{:?}", p2);
}
