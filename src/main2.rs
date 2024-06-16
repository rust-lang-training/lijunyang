fn main() {
    let b = String::from("1 2 3 4  5   6    7");
    for item in b.trim().split(" ") {
        println!("{}", item);
    }
    let mut str = String::new();
    println!("请输入5个数字，用空格隔开：");
    std::io::stdin().read_line(&mut str).expect("输入错误");
    let iter = str.trim().split(" ");
    let mut arr: [u32; 5] = [0; 5];
    let mut count = 0;
    for item in iter {
        println!("{}", item);
        if count == 5 {
            break;
        }

        arr[count] = item.parse().expect("解析错误");
        count += 1;
    }
    println!("输入的数字为：{:?}", arr);
    println!("sum为{}", sum(&arr));
}

fn sum(arr: &[u32; 5]) -> u32 {
    let mut sum = 0;
    for item in arr {
        sum += item;
    }
    sum
}
