
/// 输入是 Option<&str>（可能有字符串，也可能是 None）
/// 转成 Option<i32>
/// 过滤数字小于 10 的
/// 返回 Option<i32>
#[test]
fn parse_and_filter(){

    fn parse_and_filter(opt: Option<&str>) -> Option<i32> {
        // and_then的函数需要返回 Option<T>
        opt.map(|s| s.parse::<i32>().ok()) // 先尝试解析字符串成数字
            .flatten()
            .filter(|&num| num >= 10)             // 过滤小于 10 的数字
    }

    println!("{:?}", parse_and_filter(Some("20")));  // Some(20)
    println!("{:?}", parse_and_filter(Some("5")));   // None
    println!("{:?}", parse_and_filter(Some("abc"))); // None
    println!("{:?}", parse_and_filter(None));        // None
}