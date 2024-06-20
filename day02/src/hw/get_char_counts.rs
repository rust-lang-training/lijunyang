fn main() {
    println!("请输入一段文本，然后按Enter");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("输入错误！");
    let counts = get_char_counts(&input);

    for i in 0..counts.len() {
        let ch = (i + 'a' as usize) as u8 as char;
        println!("字符 {} 的个数是：{}", ch, counts[i as usize]);
    }
}

// 随机输入一段文本 统计每个字符出现的次数
// 要求 仅统计 a-z 26个字母 其他字符舍弃
// 如果输入的文本中有大写的 A-Z 按小写字母处理
//输出的时候 一行一个结果，总共输出26行，例如 a => 1

fn get_char_counts(input: &String) -> [i32; 26] {
    let mut counts: [i32; 26] = [0; 26];

    const A: char = 'A';
    const Z: char = 'Z';
    const a: char = 'a';
    const z: char = 'z';
    for s in input.chars() {
        if (s >= a && s <= z) || (s >= A && s <= Z) {
            let index = s.to_lowercase().next().unwrap() as i32 - a as i32;
            counts[index as usize] += 1;
        }
    }

    counts
}
