use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    // ?演算子は返り値がResultの時に使える
    // エラーをこの関数の呼び出し元に移譲している
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 型の中に制約を押し込むのがRust流
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    let _ = read_username_from_file();
}
