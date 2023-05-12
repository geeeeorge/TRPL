fn main() {
    let s1 = String::from("s1");
    let mut s2 = String::from("s2");

    let len = calculate_length(&s1);
    println!("{}", len);

    change(&mut s2);
    println!("{}", s2);
}

// 不変な借用
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可変な借用
fn change(s: &mut String) {
    s.push_str(", world");
}
