struct Point<T> {
    pub x: T,
    pub y: T,
}

// 公共
impl<T: Copy> Point<T> {
    fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

// 专门为f64类型提供的方法
impl Point<f64>{
    fn distance_from_origin(&self) -> f64 {
        (self.x as f64).hypot(self.y as f64)
    }
}

#[test]
fn test_method_special_constraint(){

    let point = Point{x: 1, y: 2};
    point.to_tuple();
    // point.distance_from_origin(); // ❌ 没有提供这个方法

    let point = Point{x: 1.0, y: 2.0};
    point.to_tuple();
    point.distance_from_origin();
}