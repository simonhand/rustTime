use std::io;

fn main() {
    println!("请猜下这个数字吧");
    let mut guess_str = String::new();
    io::stdin()
        .read_line(&mut guess_str)
        .expect("fail to readString");
    println!("you guess string is {}", guess_str);
}