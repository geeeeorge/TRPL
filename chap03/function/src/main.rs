fn main() {
    // セミコロンは「式」を表す
    // セミコロンなければ「文」
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let five = five();
    let n = plus_one(five);
    println!("{}", n);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
