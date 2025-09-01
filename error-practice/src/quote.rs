use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file_v1() -> Result<String, io::Error> {
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

// 使用?简化match控制流程的处理流程
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();
    // File
    let mut username_file = File::open("hello.txt")?;

    // 这里不需要变量接收，只需要处理Error的情况
    username_file.read_to_string(&mut username)?;

    Ok(username)
}

// ?允许链式调用
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
