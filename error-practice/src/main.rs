mod quote;
mod our_error;
mod question_with_option;


use std::io::{ErrorKind};



use std::fs::File;
use std::error::Error;
// fn main() -> Result<(),Box<dyn Error>>{
//     println!("hello world");
//     let _file = File::open("some_file.txt")?;
//     Ok(())
// }


fn main(){
    panic!("panic");
    println!("hello world")
}
#[test]
fn main_test() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}



fn test_sugar() -> Result<String,Box<dyn Error>> {

    let e: Result<String, Box<dyn Error>> = Ok("Pkmer".to_string());
    let result = e?;
    Ok(result)
}