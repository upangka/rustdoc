#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 这里返回的不是引用，和生命周期没有关系
    fn level(&self) -> i32 {
        3
    }

    // 第三条规则运用
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    // 实际上需要这样写：
    fn compare<'b>(&'a self, other: &'b str) -> &'a str
    where
        'b: 'a, // other的生命周期至少要和self一样长
    {
        if self.part.len() > other.len() {
            self.part
        } else {
            other // 这样other可以被转换为&'a str
        }
    }
}

#[test]
fn test() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{i:?}");

    println!("result = {}", i.compare("abc")); // result = Call me Ishmael
}
