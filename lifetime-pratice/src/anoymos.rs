// fn foo(x: &'_ str, y: &'_ str) -> &'_ str {
//     x
// }

// fn foo(s: &'_ str) -> &'_ str { s }

// fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
// where
//     'b: 'a,  // y 的生命周期至少要和 x 一样长
// {
//     if x.len() > y.len() { x } else { y }
// }

fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display, // Trait约束
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
