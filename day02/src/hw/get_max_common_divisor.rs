fn main() {
    assert_eq!(3, get_max_common_divisor(21, 24));
    assert_eq!(10, get_max_common_divisor(100, 70));
    assert_eq!(30, get_max_common_divisor(150, 90));
    println!("求两个整数的最大公约数");
    println!("请输入第一个整数,然后按 Enter:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("输入错误");
    println!("请输入第二个整数,然后按 Enter:");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("输入错误");
    let n1: u32 = input1.trim().parse().expect("第一个整数输入错误");
    let n2: u32 = input2.trim().parse().expect("第一个整数输入错误");
    let max = u32::MAX as u64;
    if n1 as u64 > max {
        panic!("第一个整数超出范围");
    }
    if n2 as u64 > max {
        panic!("第二个整数超出范围");
    }
    println!(
        "{} 和 {} 的最大公约数是: {}",
        n1,
        n2,
        get_max_common_divisor(n1, n2)
    );
}
// 让用户输入2个 2^31 - 1 以内的正整数，计算其最大公约数
// 推荐使用辗转相除法
fn get_max_common_divisor(a: u32, b: u32) -> u32 {
    let mut max = a.max(b);
    let mut min = a.min(b);

    loop {
        let rem = max % min;
        if rem == 0 {
            return min;
        }
        max = min;
        min = rem;
    }
}
