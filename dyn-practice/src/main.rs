mod animal;
use crate::animal::{Animal,Sheep,Cow};
fn main() {
    let random_number = 0.234;
    // let animal = random_animal(random_number);
    // println!("You've randomly chosen an animal, and it says {}", animal.noise());

    println!("\n=== 内存大小对比 ===");
    println!("Sheep size: {} bytes", std::mem::size_of::<Sheep>());
    println!("Cow size: {} bytes", std::mem::size_of::<Cow>());
    println!("Box<dyn Animal> size: {} bytes", std::mem::size_of::<Box<dyn Animal>>());
    // Box<dyn Animal> 的大小是固定的，不管里面装的是什么类型
}


// fn random_animal(random_number: f64) -> Box<dyn Animal>{
//     if random_number < 0.5{
//         // Sheep{}
//         Box::new(Sheep{})
//     }else {
//         Box::new(Cow{})
//     }
// }