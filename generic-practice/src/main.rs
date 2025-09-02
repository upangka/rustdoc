fn main() {
    let vec = vec![34, 50, 25, 100, 65];
    let array = [1, 2, 3, 4];

    // &[i32]
    let slice = &vec[0..2];
    // &[i32]
    let slice = &array[0..2];

    // 自动解引用转换 (Deref coercion)
    // &Vec<i32>
    let slice: &[i32] = &vec;
    // &[i32:4]
    let slice: &[i32] = &array;
}

fn largest_i32(list: &[i32]) -> &i32 {
    // &i32
    let mut largest = &list[0];

    // item 也是 &i32
    for item in list {
        // 自动解引用，不需要这样写 *item > *largest
        if item > largest {
            largest = item;
        }
    }

    largest
}
