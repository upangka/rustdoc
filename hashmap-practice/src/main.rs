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


#[test]
fn update_element(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores); // {"Blue": 20}
}


#[test]
fn test_entry(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // "Yellow"不存在，插入“Yellow”-> 50 健值对
    scores.entry(String::from("Yellow")).or_insert(50);
    // "Blue"已存在，不做任何处理
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
}

#[test]
fn test_update_base_old_value(){
    let text = String::from("hello world wonderful world");
    // 类型的推断 HashMap<&str,i32>
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        // &mut i32
       let count =  map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map)
}
// output:
// {
// "hello": 1,
// "wonderful": 1,
// "world": 2,
// }


#[test]
fn test_access_by_key(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let v1 = &scores["Blue"];
    println!("{v1}");

    // 当key不在的时候会直接 panic
    let v1 = &scores["Red"];
    println!("{v1}")
}