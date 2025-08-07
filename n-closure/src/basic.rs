use std::fs::File;

#[test]
fn test_format() {
    let mut a = Box::new(1);
    *a += 1;
    let b = &mut *a;
    *b += 1;
    println!("a = {}", a); // a = 3
    fn greet(g: &mut String) {
        g.push_str(" World");
    }

    let mut s = String::from("Hello");
    greet(&mut s);
    println!("{}", s);
    let s1 = &mut s;
    s1.push_str(" World");
    println!("{}", s);
}

fn test_pointer() {
    // 使用Box创建了一个指针
    let x: Box<i32> = Box::new(3);

    // 从指针可以获得引用
    let y: &Box<i32> = &x;

    // 也可以直接获得指针指向的具体数据
    let z: i32 = *x;

    // 引用本事也是指针也可以解引用，这里需要添加两个星号，来获取真实的数据
    let z1: i32 = **y;

    // 解指针，然后直接引用数据
    let z2: &i32 = &*x;
}

fn test_implicitly() {
    let s = Box::new("hello");
    let a = "xxx";
    str::len(a);
}

#[test]
fn box_aliasing_mutation_conflict() {
    let mut data = Box::new(42);

    let r1 = &*data; // 不可变引用 r1（别名）
    // let r2 = &mut *data; // ❌ 编译错误：不能在存在不可变引用时再创建可变引用
    println!("r1: {}", r1);
}

#[test]
fn box_mutation_without_aliasing() {
    let mut data = Box::new(42);
    let r2 = &mut *data; // 可变引用 r2（别名）
    *r2 += 1;

    println!("r2: {} ", r2);
    *data += 1; // ❌ 等价于 *(&mut *data) += 1; 这里触发二次可变借用
    //println!("r2: {} ", r2);
}

fn test_alias() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2]; // 别名: 不可变借用
    // v.push(4); // ❌ 编译错误：不能在存在不可变引用时再修改数据
    println!("Third element is {}", *num);
}


fn test_mut(){
    let x = 10;
    let mut y1 = &x; // y1 是可变的引用变量，但引用的是不可变的 x

    println!("y1 = {}", y1);

    // y1 可以指向别的不可变引用
    let a = 20;
    y1 = &a;

    println!("y1 now points to a = {}", y1);

    // 不能通过 y1 修改数据，下面编译不过
    // *y1 += 1;
}

#[test]
fn test_vec(){
    let mut v = vec![1, 2, 3];
    let num = &v[0];
    let num2 = &v[0];

}