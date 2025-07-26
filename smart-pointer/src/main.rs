struct Sheep {
    pub name: String,
}

impl Sheep {
    fn new(name: String) -> Self {
        Sheep { name }
    }

    fn noise(self: &Self) {
        println!("{} mie mie mie", self.name);
    }
}

fn main() {
    let sheep1 = Sheep::new("sheep1".to_string());
    sheep1.noise();
    // Box包裹的对象，由于它是指针，可以直接访问对象的方法
    let sheep2 = Box::new(Sheep::new("sheep2".to_string()));
    sheep2.noise();
}
