mod option_test;

use serde_json::{Result, Value};

const DATA: &str = r#" { "name": "John Doe", "age": 43}  "#;
// static DATA: &str = r#" { "name": "John Doe", "age": 43 } "#;

/// 使用match来处理Result
#[test]
fn test_basic_1() {
    // from_str返回的是Result
    let v: Result<Value> = serde_json::from_str(&DATA);

    match v {
        Ok(value) => {
            println!("{:#?}", value);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

/// 使用unwrap来处理Result
#[test]
fn test_basic_2() {
    let v: Value = serde_json::from_str(&DATA).unwrap();
    println!("{:#?}", v);
}

/// 使用expect来处理Result
#[test]
fn test_basic_3() {
    let v: Value = serde_json::from_str(&DATA).expect("JSON解析错误❌");
    println!("{:#?}", v);
}

/// 使用?来处理Result
/// 最优雅的处理方式
#[test]
fn test_basic_4() -> Result<()> {
    let v: Value = serde_json::from_str(&DATA)?;
    println!("{:#?}", v);
    Ok(())
}
