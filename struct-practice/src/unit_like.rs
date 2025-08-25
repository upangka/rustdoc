struct Point { x: i32, y: i32 }

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

#[test]
fn test_main() {
    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;      // (1) 对 p.x 的可变借用
    print_point(&p);       // (2) 对整个 p 的不可变借用
    // *x += 1;               // (3) 修改 x (也就是 p.x)
}
