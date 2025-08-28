mod derive_trait;
mod unit_like;

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

struct Color(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;

    println!("{}", black.0);

    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    // println!("{} {}", *x, p.y);
    println!("{}", *x);
    println!("{}", p.y);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
