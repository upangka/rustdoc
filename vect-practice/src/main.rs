fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        _ => (),
    }

    println!("The third element is {:?}", third);
}

#[test]
fn test_no_exists() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = v.get(100);
    println!("{does_not_exist:?}"); // None

    let does_not_exist = &v[100]; // 直接报错，终止程序运行
}

#[test]
fn test_iterator() {
    let v = vec![100, 32, 57];

    // i 的类型是一个引用i32类型 (&i32)
    for i in &v {
        println!("{i}");
    }

    println!("{:?}", v);
}

#[test]
fn test_mut_iterator() {
    let mut v = vec![100, 32, 57];
    // i此时的类型为 &mut i32
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

#[test]
fn test_string_vec(){
    let mut v = vec![String::from("Hello")];
    let word = String::from(" World");

    // 从v中取出字符串可以直接追加字符串，因为v声明了mut,
    // 不仅v本身可以修改，它里面的元素也可以变得可以修改
    v[0].push_str(&word);

    // 而单独的字符串要修改需要显示地声明为mut
    let mut word = String::from(" World");
    word.push_str("!");
}

#[test]
fn test_mut_element(){
    let mut v = vec![String::from("Hello")];
    let s = &mut v[0];

    s.push_str(" World");

    // let s = v[0];  // error String类型没有实现Copy trait

    println!("{v:?}"); // ["Hello World"]
}