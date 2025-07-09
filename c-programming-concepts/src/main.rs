mod shadowing;
mod tuple;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    // mut variables
    println!("The value of x is: {}", x);
}
