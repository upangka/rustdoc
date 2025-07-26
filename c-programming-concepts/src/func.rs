fn five() -> i32 {
    // expression
    5
    // statement
    // return 5;
}

#[test]
fn test() {
    let x = five();
    println!("x is {}", x);
}
