#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn get_color(&self) -> &str {
        match self {
            Color::Red => "#FF0000",
            Color::Green => "#00FF00",
            Color::Blue => "#0000FF",
        }
    }
}

fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("found: {}", number),
        None => println!("Nothing found"),
    }
}

fn main() {
    let color = Color::Red;
    println!("color: {:?}", color);
    println!("color code: {}", color.get_color());

    find_maybe_number(Some(3));
    find_maybe_number(None);

    // if let文
    let some_number: Option<u32> = Some(5);
    if let Some(number) = some_number {
        println!("some_number is : {}", number);
    } else {
        println!("some_number is : None");
    }

}
