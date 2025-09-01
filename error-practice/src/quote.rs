use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // Result<File>
    let username_file_result = File::open("hello.txt");

    // File
    let mut username_file = match username_file_result {
        Ok(file) => file,                // 返回file给username_file变量
        Err(error) => return Err(error), // 直接结束函数
    };
    let mut username = String::new();

    // 注意这里可以直接返回值，简化return
    // read_to_string 返回的是Result<usize>
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username), // _占位符，不绑定值
        Err(error) => Err(error),
    }
}
