#[test]
fn test_one() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        Some(_) => println!("Some!"),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}

#[test]
fn test_fix_two() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt); 
}
