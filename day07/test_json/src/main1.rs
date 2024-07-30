use serde::{Deserialize, Serialize};

extern crate serde;
extern crate serde_json;

fn main() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        // 使用Serde的派生属性宏来定制序列化行为，
        // 改变JSON中的字段名称
        #[serde(rename = "fullName")]
        name: String,
        // 使用派生属性来定制反序列话行为
        // 为缺失的字段提供默认值
        #[serde(default = "default_age")]
        age: u8,
    }

    fn default_age() -> u8 {
        18
    }
    let john = Person {
        name: String::from("John"),
        age: 30,
    };
    // 序列化 将Rust的数据结构转化为 JSON格式的字符串
    // serde_json::to_string()
    let str1 = serde_json::to_string(&john).unwrap();
    println!("{}", str1);

    let str2 = r#"{"fullName":"John", "age":30}"#;
    // 反序列化 将JSON格式的字符串转化为 Rust的数据结构
    // serde_json::from_str()
    let john2 = serde_json::from_str::<Person>(&str2).unwrap();
    println!("{:?}", john2);

    let str3 = r#"{"fullName":"lucy"}"#;
    let lucy = serde_json::from_str::<Person>(&str3).unwrap();
    println!("{:?}", lucy);

    let str4 = r#"{"fullName":"lucy", "age": 20}"#;
    // serde_json::Value 处理动态结构的JSON数据， 是一个枚举，用于存储任意的JSON数据
    let v: serde_json::Value = serde_json::from_str(&str4).unwrap();
    println!("{:?}", v);
    // 使用 json! 宏 来创建json数据
    let john = serde_json::json!({"fullNae": "john", "age": 30});
    println!("{:?}", john);

    // 错误处理
    let res = serde_json::from_str::<Person>(r#"{"age": 30}"#);
    if let Err(e) = res {
        println!("Error: {}", e);
    }
}
