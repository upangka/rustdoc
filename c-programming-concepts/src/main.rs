mod shadowing;
mod tuple;
mod func;
mod reference;
mod mut_reference;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    // mut variables
    println!("The value of x is: {}", x);
}
