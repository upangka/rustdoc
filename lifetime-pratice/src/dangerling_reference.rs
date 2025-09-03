#[test]
fn dangling_references_demo() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("inner scope r: {r}"); // ✅
    }

    // println!("outer scope r: {r}"); // ❌
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[test]
fn lifetime_annotation_in_fn_signature() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(&string1, string2);
        println!("The longest string is {}", result)
    }
}

#[test]
fn test_as_str(){
    let s = String::from("foo");
    assert_eq!(&s, s.as_str());
}