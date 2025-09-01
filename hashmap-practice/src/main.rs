use std::collections::HashMap;

fn main() {
    // 不需要显示指定类型，编译器会自动推导
    let mut scores = HashMap::new();
    // 插入数据的时候会自动推到出来
    scores.insert(String::from("Blue"), 2);
    scores.insert(String::from("Yellow"), 5);

    let team_name = String::from("Blue");
    // 这里get 返回的是一个 Option<&i32>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}"); // 2

    // 防止scores所有权被移动，这里使用 &
    for (key,value) in scores{
        println!("{key}:{value}")
    }
    // error Value used after being moved
    // println!("{scores:#?}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // move here
    // println!("{field_name} {field_value}"); // 所有权已经被转移

}


#[test]
fn test_reference_value(){
    let mut map = HashMap::new();
    {
        let key = String::from("a");
        map.insert(&key, 1);
        println!("{key}");
    }

    // 此时会报错，原因是因为key的存活时间比map短
    // 上面的作用域结束之后，key就消失了
    // println!("{:?}", map)
}