#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn show_this_message(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    // Enum
    let mut m = Message::Quit;
    m.show_this_message();

    m = Message::Move { x: 10, y: 20 };
    m.show_this_message();

    m = Message::Write(String::from("Hello"));
    m.show_this_message();

    m = Message::ChangeColor(255, 0, 0);
    m.show_this_message();

    // Option Enum
    let mut some_number = Some(5);
    println!("{:?}", some_number);
    some_number = None;
    println!("{:?}", some_number);
}
