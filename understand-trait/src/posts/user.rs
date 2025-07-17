pub struct User {
    pub name: String,
}

impl User {
    pub fn hello(&self) {
        println!("hello,{}", self.name);
    }
}

pub trait Greet {
    fn greet(&self);
}

impl Greet for User {
    fn greet(&self) {
        println!("Greet,{}", self.name);
    }
}
