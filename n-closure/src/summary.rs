#[test]
fn main() {
    let mut a_num = 0;
    inner(&mut a_num);
    println!("{}", a_num)
}

fn inner(x: &mut i32) {
    let another_num = 1;
    let a_stack_ref = &another_num;

    let mut a_box = Box::new(2);
    let a_box_stack_ref = &a_box;
    let a_box_heap_ref = &mut *a_box; // 唯一引用

    *a_box_heap_ref += 6;
    println!("a_box_heap_ref = {a_box_heap_ref}"); // a_box_heap_ref = 8
    println!("a_box = {a_box}"); // a_box = 8
    *x += 5;
}

fn test_drop() {
    let v = vec![1, 2, 3];
    let v_ref: &Vec<i32> = &v;
    // *v_ref操作会尝试移动所有权
    // let v2 = *v_ref;
}
