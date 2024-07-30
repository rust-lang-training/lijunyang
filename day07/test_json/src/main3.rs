use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_str, to_string};
#[derive(Debug, Serialize, Deserialize)]
struct Animal {
    #[serde(
        serialize_with = "serialize_name",
        deserialize_with = "deserialize_name"
    )]
    name: String,
    age: u32,
}
fn serialize_name<S>(name: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&name.to_uppercase())
}
fn deserialize_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let name = String::deserialize(deserializer)?;
    Ok(name.to_lowercase())
}

fn test1() {
    let a = Animal {
        name: "Tom".to_string(),
        age: 10,
    };
    let str1 = to_string(&a).unwrap();
    println!("{}", str1);
    let str2 = r#"{"name":"TOM","age":10}"#;
    let a = from_str::<Animal>(&str2).unwrap();
    println!("{:?}", a);
}
fn main() {
    test1();
}
