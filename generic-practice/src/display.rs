use std::fmt::Display;
fn displayable<T: Display>(t: T) -> impl Display{
    t
}

#[test]
fn test_display(){
    let s = String::from("hello");
    // 此时s1是impl Display 代表某个实现了Display trait的类型
    let mut s1  = displayable(s);

    // 此时s1不能调用push_str，因为rust并不知道它是一个String字符串类型
    // 只知道它是某个实现了Display trait的类型
    // s1.push_str(" word"); // ❌
}
//
// use std::fmt::Display;
//
// fn displayable<T: Display>(t: T) -> T {
//     t
// }
//
// #[test]
// fn test_display() {
//     let s = String::from("hello");
//     let mut s1 = displayable(s);
//
//     s1.push_str(" world"); // ✅ 现在可以了
//     println!("{}", s1);
// }
