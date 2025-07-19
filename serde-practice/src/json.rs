use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[test]
fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "Pkmer",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // ？处理Result
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("请使用 {} 电话打给 {}", p.phones[0], p.name);

    Ok(())
}
