// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),               // 携带字符串的变体
    Move { x: i32, y: i32 },    // 携带匿名结构体的变体
    ChangeColor(u8, u8, u8),    // 携带三个 u8 类型（RGB）的变体
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello")));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(255, 0, 0));
}