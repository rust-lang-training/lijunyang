use serde::{Deserialize, Serialize};
use serde_json::{
    from_reader, from_slice, from_value, to_string, to_string_pretty, to_value, to_vec,
    to_vec_pretty, to_writer, to_writer_pretty, Value,
};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
}
fn test_reader() {
    let file = File::open("person.json").unwrap();
    // 从 I/O流中反序列化一个json对象
    let p: Person = from_reader(file).unwrap();
    println!("{:?}", p);
}
fn test_slice() {
    let str1 = r#"{"name": "John", "age": 30}"#;
    // 从字节片段中反序列化一个json对象
    let p: Person = from_slice(str1.as_bytes()).unwrap();
    println!("{:?}", p);
}

fn test_value() {
    let v: Value = serde_json::json!({ "name": "John", "age": 30 });
    // 从 serde_json::Value 中解析一个对象
    let p: Person = from_value(v).unwrap();
    println!("name: {} age: {}", p.name, p.age);
}

fn test_to_string() {
    let p = Person {
        name: String::from("John"),
        age: 30,
    };
    // 序列化一个对象为json字符串
    let data = to_string::<Person>(&p).unwrap();
    println!("{:?}", data);
    // 序列化一个对象为json字符串(带缩进)
    let data2 = to_string_pretty::<Person>(&p).unwrap();
    println!("{:?}", data2);
}

fn test_to_value() {
    let p = Person {
        name: String::from("John"),
        age: 30,
    };
    // 将一个类型转换为 serde_json::Value
    let v = to_value::<Person>(p).unwrap();
    println!("{:?}", v)
}

fn test_to_vec() {
    let p = Person {
        name: String::from("John"),
        age: 30,
    };
    // 将一个对象序列化为带缩进的json字节向量
    let j = to_vec::<Person>(&p).unwrap();
    println!("{:?}", j);
    // 将一个对象序列化为json的字节向量
    let jp = to_vec_pretty::<Person>(&p).unwrap();
    println!("{:?}", jp);
}
fn test_writer() {
    let p = Person {
        name: String::from("John"),
        age: 30,
    };
    let writer = File::create("person2.json").unwrap();
    // 将对象序列化为json并写入到 I/O流
    to_writer(writer, &p).unwrap();
    let writer2 = File::create("person3.json").unwrap();
    to_writer_pretty(writer2, &p).unwrap();
}
fn main() {
    // test_reader();
    // test_slice();
    // test_value();
    // test_to_string();
    // test_to_value();
    // test_to_vec();
    test_writer();
}
