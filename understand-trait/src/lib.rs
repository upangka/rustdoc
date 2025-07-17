mod posts;
// 重新导出
pub use posts::*;

pub trait HelloExt {
    fn hello(&self);
}

impl HelloExt for String {
    fn hello(&self) {
        println!("hello {}", self);
    }
}
