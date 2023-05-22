use std::cmp::PartialOrd;

// その型Tにできてほしいことを保証する
// ようにトレイト境界を使う
fn max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &number in list {
        if number > max {
            max = number
        }
    }
    max
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // trait
    let numbers = vec![34, 50, 25, 100];
    println!("The largest number is {}", max(&numbers));

    let numbers = vec![34.1, 5.3, 25.2, 100.5];
    println!("The largest number is {}", max(&numbers));

    // lifetime
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
