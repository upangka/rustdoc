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
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x as f64).hypot(self.y as f64)
    }
}

#[test]
fn test_method_special_constraint() {
    let point = Point { x: 1, y: 2 };
    point.to_tuple();
    // point.distance_from_origin(); // ❌ 没有提供这个方法

    let point = Point { x: 1.0, y: 2.0 };
    point.to_tuple();
    point.distance_from_origin();
}

#[test]
fn change_array_vec() {
    let v = vec![1, 2, 3];
    let array = [1, 2, 3, 4];

    // 数组切片转换为Vec
    let slice = &array[..];
    let result = slice.to_vec();

    // Vec转换为数组切片
    let slice: &[i32] = &v;
    let result = slice.to_vec();

    // 数组转换成数组
    let result = array.to_vec();
}

#[derive(Debug)]
struct TestResult {
    scores: Vec<usize>,

    curve: Option<usize>,
}

impl TestResult {
    pub fn get_curve(&self) -> &Option<usize> {
        &self.curve
    }

    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.curve {
            // score类型&mut usize
            for score in &mut self.scores {
                *score += curve;
            }

            // score类型&mut usize
            for score in self.scores.iter_mut() {
                *score += curve;
            }
        }
    }
}

#[test]
fn test_apply_curve() {
    let mut test_result = TestResult {
        scores: vec![1, 2, 3],
        curve: Some(1),
    };
    test_result.apply_curve();
    println!("{:?}", test_result) // TestResult { scores: [3, 4, 5], curve: Some(1) }
}


#[test]
fn test_vec_iter_mut(){
    let mut v = vec![100, 32, 57];
    // i此时的类型为 &mut i32
    for i in &mut v {
        *i += 50;
    }

    // 等效

    // i此时的类型为 &mut i32
    for i in v.iter_mut() {
        *i += 50;
    }
    println!("{:?}", v);
}