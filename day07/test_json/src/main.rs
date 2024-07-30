use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
enum Animal {
    Dog { name: String, age: u32 },
    Cat { name: String, age: u32 },
}

fn test() {
    let dog = Animal::Dog {
        name: "Tom".to_string(),
        age: 33,
    };
    println!("{:?}", dog);
    let j = to_string(&dog).unwrap();
    println!("{}", j);

    let str = r#"{"Cat":{"name":"Tom","age":33}}"#;
    let x = from_str::<Animal>(str).unwrap();
    println!("{:?}", x);
}

fn test2() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Animal {
        // 指定Option 值为null时不进行序列化
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        age: u32,
    }

    let a = Animal {
        name: Some("Tom".to_string()),
        age: 33,
    };
    let j = to_string::<Animal>(&a).unwrap();
    println!("{:?}", j);

    let b = Animal {
        name: None,
        age: 33,
    };
    let j = to_string::<Animal>(&b).unwrap();
    println!("{:?}", j);

    let j = r#"{"age": 33}"#;
    let x = from_str::<Animal>(&j).unwrap();
    println!("{:?}", x);
}

fn test3() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Animal {
        name: String,
        age: u32,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Zoo {
        animals: Vec<Animal>,
    }

    let zoo = Zoo {
        animals: vec![
            Animal {
                name: "tom".to_string(),
                age: 33,
            },
            Animal {
                name: "jerry".to_string(),
                age: 8,
            },
        ],
    };

    let j = to_string(&zoo).unwrap();
    println!("{}", j);

    let str = r#"{"animals":[{"name":"lucy","age":3},{"name":"xiaoming","age":1}]}"#;
    let j = from_str::<Zoo>(str).unwrap();
    println!("{:?}", j.animals[0].name);
}

fn test4() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Animal {
        name: String,
        age: u32,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Zoo {
        animals: HashMap<String, Animal>,
    }

    let mut animals = HashMap::<String, Animal>::new();
    animals.insert(
        "Tom".to_string(),
        Animal {
            name: "Tom".to_string(),
            age: 33,
        },
    );
    animals.insert(
        "Jerry".to_string(),
        Animal {
            name: "Jerry".to_string(),
            age: 8,
        },
    );

    let zoo = Zoo { animals };
    let j = to_string::<Zoo>(&zoo).unwrap();
    println!("{}", j);

    let str2 = r#"{"animals":{"Jerry":{"name":"Jerry","age":8},"Tom":{"name":"Tom","age":33}}}"#;

    let j = from_str::<Zoo>(str2).unwrap();
    println!("{:?}", j.animals["Tom"].name);
}
fn main() {
    // test();
    // test2();
    // test3();
    test4();
}
