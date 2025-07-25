mod json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {}

/// 基础反序列化 JSON 为 struct
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u8,
}

#[test]
fn basic() {
    let json = r#"{"name": "Alice", "age": 30}"#;
    let user: User = serde_json::from_str(json).unwrap();
    println!("{:?}", user);
}

/// 缺失字段 + #[serde(default)]
#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    debug: bool,

    #[serde(default)]
    env_vars: HashMap<String, String>,
}

#[test]
fn test_default() {
    let json = r#"{}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    println!("{:?}", config);
}

/// 结构体整体默认值（需要 derive(Default)）
#[derive(Debug, Deserialize, Default)]
struct AppConfig {
    #[serde(default)]
    debug: bool,
    #[serde(default)]
    port: u16,
}

#[derive(Debug, Deserialize)]
struct ApiRequest {
    #[serde(default)] // 👈 整体默认值
    config: AppConfig,
}

#[test]
fn test_struct() {
    let json = r#"{}"#;
    let req: ApiRequest = serde_json::from_str(json).unwrap();
    println!("{:?}", req);
}

/// 🛠️ Demo 5：字段重命名 + 忽略字段
#[derive(Debug, Deserialize)]
struct Person {
    #[serde(rename = "userName")]
    name: String,

    #[serde(skip)]
    secret: String,
}

#[test]
fn test_rename() {
    let json = r#"{"userName": "John"}"#;
    let person: Person = serde_json::from_str(json).unwrap();

    println!("{:?}", person);
}

#[derive(Serialize)]
struct Student {
    id: u32,
    username: String,
    active: bool,
}

#[test]
fn test_serialize() {
    let user = Student {
        id: 1,
        username: "du_xiao".to_string(),
        active: true,
    };

    let json_str = serde_json::to_string(&user).unwrap();
    println!("{}", json_str);
}

use serde_json::{Result, Value};
#[test]
fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
