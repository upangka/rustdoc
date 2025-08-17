#[test]
fn test_basic() {
    let s = String::from("Hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    println!("{slice}");

    let slice = &s[3..s.len()];
    // 等价
    let slice = &s[3..];
    println!("{slice}");
}

#[test]
fn test_first_word() {
    let mut s = String::from("hello world");
    let slice = first_word(&s);
    s.clear(); // 报错，s需要RW权限
    // println!("{slice}") // 此时slice生命周期还没有结束，权限还没有归还
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if (item == b' ') {
            return &s[..i];
        }
    }
    &s[..]
}

fn test_other_slice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
}

#[test]
fn test_slice_size(){
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
    // &String=8 &str=16
}
