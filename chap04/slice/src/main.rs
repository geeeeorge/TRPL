fn main() {
    let mut s = "hello world";

    let word = first_word(&s);
    println!("{}", word);

    // スライスは不変な参照, s.clear()は可変なのでCE
    // s.clear();
    // println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}