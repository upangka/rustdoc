fn main() {
    println!("{} {}", "A".len(), String::from("A").chars().count()); // 1 1
    println!("{} {}", "你好".len(), "你好".chars().count()); // 6 2
    println!("{} {}", "😊".len(), "😊".chars().count()); // 4 1

    // s 的类型rust推到为String
    let s = "initial contents".to_string();
}

#[test]
fn test_push() {
    let mut s = String::from("你好");
    // 追加一个字符
    s.push('!');
    s.push('😊');
    s.push('爱');

    // 追加字符串
    s.push_str("Hello World");

    println!("{s}"); // 你好!😊爱Hello World
}

#[test]
fn test_add_operator() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // 此时s1,s2,s3都还是有效的
    println!("{s} {s1} {s2} {s3}"); // tic-tac-toe tic tac toe
}

#[test]
fn slice_string() {
    let s = String::from("Rust一门赋予每个人");

    // 按字符迭代（char 表示一个 Unicode 标量值）
    println!("按字符迭代:");
    for c in s.chars() {
        println!("{}", c);
    }

    println!("\n按字节迭代:");
    // 按字节迭代（每个字节 u8，UTF-8 编码）
    for b in s.bytes() {
        println!("{}", b);
    }

    // 原字符串
    println!("\n原字符串:\n{}", s);
}
