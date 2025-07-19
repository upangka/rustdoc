use serde_json::{Result,Value};


fn get_username(json: &Value) -> Option<&str> {
    json.get("user")?
        .get("name")?
        .as_str()
}

#[test]
fn test_basic1() {
    let data = r#"{ "user": { "name": "Alice" } }"#;
    let json: Value = serde_json::from_str(data).unwrap();
    
    match get_username(&json) {
        Some(name) => println!("用户名是：{}", name),
        None => println!("没有找到用户名"),
    }
}
