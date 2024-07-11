use std::{
    cmp::Ordering,
    fmt::{format, Display},
    io::{self, Write},
};

use rand::Rng;

const NAMES_CONTENT: &'static str = include_str!("names.txt");

#[derive(Debug, Clone)]
struct ExampleRecord {
    id: u64,
    name: String,
    scores: Vec<Option<f32>>,
    total_score: Option<f32>,
    avg_score: Option<f32>,
}

impl Display for ExampleRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "2024-{:03}     {:8}      {:>6}       {:>6}       {:>6}       {:>7}       {:>6}",
            self.id,
            &self.name,
            match &self.scores[0] {
                Some(s) => format!("{:.2}", s),
                None => "".to_string(),
            },
            match &self.scores[1] {
                Some(s) => format!("{:.2}", s),
                None => "".to_string(),
            },
            match &self.scores[2] {
                Some(s) => format!("{:.2}", s),
                None => "".to_string(),
            },
            match &self.total_score {
                Some(s) => format!("{:.2}", s),
                None => "".to_string(),
            },
            match &self.avg_score {
                Some(s) => format!("{:.2}", s),
                None => "".to_string(),
            }
        )
    }
}
impl ExampleRecord {
    fn gen_records() -> Vec<ExampleRecord> {
        let mut id = 0u64;
        NAMES_CONTENT
            .split("\n")
            .filter(|s| !s.is_empty())
            .take(100)
            .map(|s| {
                let mut rng = rand::thread_rng();
                let n = rng.gen_range(1..=100);
                let chinese_score = if (id + 1) % 10 == 0 && n % 7 == 0 {
                    None
                } else {
                    let score: f32 = rng.gen();
                    let score = 40.0 + score * 60.0;
                    Some(score)
                };
                let math_score = if (id + 1) % 10 == 0 && n % 7 == 0 {
                    None
                } else {
                    let score: f32 = rng.gen();
                    let score = 40.0 + score * 60.0;

                    Some(score)
                };
                let english_score = if (id + 1) % 10 == 0 && n % 7 == 0 {
                    None
                } else {
                    let score: f32 = rng.gen();
                    let score = 40.0 + score * 60.0;

                    Some(score)
                };
                let scores = vec![chinese_score, math_score, english_score];

                let attend_exams: Vec<&Option<f32>> =
                    scores.iter().filter(|s| s.is_some()).collect();

                let (total_score, avg_score) = if attend_exams.is_empty() {
                    (None, None)
                } else {
                    let total: f32 = attend_exams.iter().map(|s| s.unwrap()).sum();
                    (Some(total), Some(total / attend_exams.len() as f32))
                };
                id += 1;
                ExampleRecord {
                    id,
                    name: s.to_owned(),
                    scores,
                    total_score,
                    avg_score,
                }
            })
            .collect()
    }
}

fn sort_by_total_score(records: &mut Vec<ExampleRecord>) {
    records.sort_by(|a, b| match (a.total_score, b.total_score) {
        (Some(sa), Some(sb)) => sb.total_cmp(&sa),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    })
}

fn main() {
    let mut records = ExampleRecord::gen_records();

    // println!("{:?}", records);
    // println!("{:?}", NAMES_CONTENT);
    sort_by_total_score(&mut records);
    loop {
        print_main_menu();
        print!("\n请选择 [1-5]: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "5" => break,
            _ => {
                println!("  #         学  号        姓名        语文        数学        英语        总分        平均分");
                for (i, item) in records.iter().enumerate() {
                    println!("{:03}        {}", i, item);
                }
            }
        }
        sort_by_total_score(&mut records);
        for (i, item) in records.iter().enumerate() {
            println!("{:03}        {}", i, item);
        }
    }
}

fn print_main_menu() {
    println!("\n-------- 成绩查询系统 --------");
    println!("  1. 打印成绩单");
    println!("  2. 排序成绩单");
    println!("  3. 查找成绩单");
    println!("  4. 全班平均成绩");
    println!("  5. 退出");
}
