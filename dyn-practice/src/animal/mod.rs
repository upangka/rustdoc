pub struct Sheep {
    id: u32,
}
pub struct Cow {}

pub trait Animal {}

#[test]
fn test_size() {
    println!("size of Sheep: {} bytes", std::mem::size_of::<Sheep>());
    println!("size of Cow: {} bytes", std::mem::size_of::<Cow>());
    println!(
        "size of Box<Sheep>: {} bytes",
        std::mem::size_of::<Box<Sheep>>()
    );
    println!(
        "size of Box<Cow>: {} bytes",
        std::mem::size_of::<Box<Cow>>()
    );
    println!(
        "size of Box<dyn Animal>: {} bytes",
        std::mem::size_of::<Box<dyn Animal>>()
    );
}

// impl Animal for Sheep{
//     fn noise(&self)->&'static str{
//         "mie mie mie!"
//     }
// }

// impl Animal for Cow{
//     fn noise(&self)->&'static str{
//         "moo moo moo!"
//     }
// }
