use std::fs::File;

#[test]
fn test_format(){
    let msg = format!("{}", "hello world");
    msg.parse().unwrap();
}