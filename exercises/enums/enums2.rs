// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // 单元变体：无关联数据
    Quit,
    // 元组变体：关联一个 String 类型的数据
    Echo(String),
    // 结构体变体：关联带命名字段的结构体数据（x、y 为 i32 类型）
    Move { x: i32, y: i32 },
    // 元组变体：关联三个 u8 类型的数据（对应 RGB 颜色值）
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
