
/**
* 创建引用的这种叫做borrowing,借用，非常形象，而不是占有
**/
#[test]
fn reference(){
    let s1 = String::from("你好世界");
    // reference 不进行ownership的转移move
    let len = calculate_len(&s1);

    println!("s1 = {s1} the length is  {len}")
}


// 定义的时候也使用&，代表s是一个引用
fn calculate_len(s: &String) -> usize{
    s.chars().count()
}