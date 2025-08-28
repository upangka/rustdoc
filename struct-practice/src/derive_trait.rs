#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
fn test_main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    dbg!(&rect1);
}
