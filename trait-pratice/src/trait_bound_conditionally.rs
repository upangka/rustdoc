use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn draw(&self) {
        println!("drawing")
    }
}

impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x is greater than or equal to y");
        } else {
            println!("x is less than y");
        }
    }
}

#[test]
fn test_trait_bound_conditionally() {
    let p = Point { x: 5, y: 10 };
    p.draw();
    p.cmp_display();

    // cmp_display 限制了Point的类型，只能是实现了Display和PartialOrd trait的类型才有
    let p = Point {
        x: vec![1, 2, 3],
        y: vec![4, 5, 6],
    };

    p.draw();
    // 因为Vec没有试下根本就没有这个方法
    // p.cmp_display();
}
