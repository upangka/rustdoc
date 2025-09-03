// fn main() {
//
//     println!("{}",reverse("Hello, world!"));
//     println!("{}",reverse_v2("Hello, world!"))
// }

mod dangerling_reference;
mod struct_demo;
mod lifetime_elision;

pub fn hello() -> &'static str {
    "Hello, World!"
}


pub fn reverse(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars().rev(){
        s.push(c);
    }
    s
}

pub fn reverse_v2(input:&str) -> String{
    input.chars().rev().collect()
}


struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}