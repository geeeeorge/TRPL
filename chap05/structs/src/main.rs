#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

fn main(){
    // 構造体
    let user1 = build_user(String::from("aaa@fff"), String::from("aaa"));
    println!("{:?}", user1);

    let user2 = User {
        email: String::from("bbb@fff"),
        username: String::from("bbb"),
        ..user1
    };

    println!("{:?}", user2);

    // タプル構造体
    let mut rect = Rectangle {width: 30, height: 50};
    println!("rect: {:?}", rect);
    println!("area: {}", rect.area());

    rect.height = 60;
    println!("rect: {:?}", rect);
    println!("area: {}", rect.area());

    let square = Rectangle::square(3);
    println!("square: {:?}", square);
    println!("area: {}", square.area());

    rect.set_width(10);
    println!("rect: {:?}", rect);
    println!("area: {}", rect.area());

    rect.set_height(20);
    println!("rect: {:?}", rect);
    println!("area: {}", rect.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
