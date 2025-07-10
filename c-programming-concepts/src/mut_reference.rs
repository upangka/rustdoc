
/**
* 引用默认是不可变的
**/
#[test]
fn mut_refence(){
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);
}

// &mut String 表示可以修改借来的值
fn change(s: &mut String){
    s.push_str(" world");
}