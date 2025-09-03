fn largest<T>(items: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut result = &items[0];
    for item in items {
        if item > result {
            result = item;
        }
    }
    result
}

struct Point<T> {
    x: T,
    y: T,
}
