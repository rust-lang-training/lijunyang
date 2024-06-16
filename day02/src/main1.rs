fn main() {
    loop {
        println!("请选择输入你需要的温度转换， 1: 华氏转摄氏， 2: 摄氏转华氏， 0: 退出");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("输入失败");
        let input: u8 = input.trim().parse().unwrap();

        if input == 1 {
            let mut t = String::new();
            println!("请输入华氏温度");
            std::io::stdin().read_line(&mut t).expect("输入失败");
            let t: f32 = t.trim().parse().unwrap();
            println!("摄氏温度为{:.2}", ss(t));
        } else if input == 2 {
            println!("请输入摄氏温度");
            let mut t = String::new();
            println!("请输入摄氏温度");
            std::io::stdin().read_line(&mut t).expect("输入失败");
            let t: f32 = t.trim().parse().expect("输入失败");
            println!("华氏温度为{:.2}", hs(t));
        } else if input == 0 {
            println!("输入错误, 只能选择输入 1 或 2");
            break;
        }
    }
}

fn hs(ss: f32) -> f32 {
    ss * 1.8 + 32.0
}

fn ss(hs: f32) -> f32 {
    (hs - 32.0) / 1.8
}
