use std::fs::File;

#[test]
fn test_format(){
    let a = Box::new(2);
    let b = a;

    println!("{}", b);
}