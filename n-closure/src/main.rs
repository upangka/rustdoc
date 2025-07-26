fn main() {
    // 没有声明类型
    let example_closure = |x| x;
    // 提前锁定闭包的类型 fn(String) -> String
    let s = example_closure(String::from("hello"));
    // 编译错误
    // let n = example_closure(5);

    let a = 3;
    println!("{} {}", a, &a);

    let list = vec![1, 2, 3];
    println!("1: {list:?}");
    println!("2: {:?}", &list);
}
