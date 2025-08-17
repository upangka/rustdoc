

#[test]
fn test_basic(){
    let s = String::from("Hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    println!("{slice}");
}