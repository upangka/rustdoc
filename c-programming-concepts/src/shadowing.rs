#[test]
fn shadowing() {
    let x = 5;
    // shadowing 直接重新声明变量
    let x = x + 1;
    println!("x is {x}");
    
    // shadowing可以改变类型
    let x = format!("hello {}", x);
    println!("x is {x}");
}
