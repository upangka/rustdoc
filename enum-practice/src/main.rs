mod match_exhaust;
mod move_ownership;

#[derive(Debug)]
enum Message {
    Quit,                       // 正常枚举
    Move { x: i32, y: i32 },    // 直接定义结构体
    Write(String),              // 接收参数
    ChangeColor(i32, i32, i32), // 接收多个参数
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main_22() {
    let message = Message::Move { x: 1, y: 2 };
    message.call();
}

fn do_something() {
    println!("Doing something...");
    // 隐式返回 ()
}

fn main() {
    let x = (); // 单位值
    println!("Unit value: {:?}", x);

    let y = do_something(); // 返回单位值
    println!("Return from function: {:?}", y); // Return from function: ()

}
