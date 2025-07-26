use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
fn main() {
    let mut guess = String::new();
    // 使用rand生成随机的数字范围1..=100

    let answer: u32 = rand::rng().random_range(1..=100);
    println!("答案为: {answer}");

    loop {
        // 清空
        guess.clear();
        println!("请输入猜测的数字");
        // 会追加到指定内存区域
        stdin().read_line(&mut guess).expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("输入错误: {}", e);
                continue;
            }
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("恭喜你猜对了");
                break;
            }
        }
    }
}
