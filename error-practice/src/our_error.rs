use std::fs::File;
use std::io::{self, Read};

// 自定异常错误
struct OurError {
    value: io::Error,
}

// 定义转换 io::Error -> OurError
impl From<io::Error> for OurError {
    fn from(value: io::Error) -> Self {
        Self { value }
    }
}

fn read_username_from_file() -> Result<String, OurError> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
