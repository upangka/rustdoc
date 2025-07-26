#[test]
fn tuple() {
    // 或自动类型推导(i32,f64,i32)
    let tup = (500, 6.4, 1);

    // 访问
    let (x, y, z) = tup;
    println!("x:{x},y:{y},z:{z}");

    // 通过下标
    println!("x:{},y:{},z:{}", tup.0, tup.1, tup.2);
}
