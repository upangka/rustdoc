mod ownership_func;

fn main() {
    // 在内存堆上分配一个空间
    let s1 = String::from("hello");
    // 在栈上复制指针ptr，length，capacity
    let s2 = s1;
    // 栈上指针被复制，此时s1已经无效了
    // 这种设计在rust中叫move，
    // 这种设计是为了rust free清理空间，不执行两次的一种设计
    // 报错：value borrowed here after move
    // println!("{}, world!", s1);
    println!("{}, world!", s2);
}
