// 声明模块
mod garden;
// 只有声明了模块才能使用use，来简化路径，否则会报错
use crate::garden::{grow, vegetables::Asparagus};

fn main() {
    grow();
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
