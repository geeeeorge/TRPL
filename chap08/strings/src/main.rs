fn main() {
    // push_strはstring, pushはchar
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    println!("s1 = {}", s1);

    let mut s2 = String::from("lo");
    s2.push('l');
    println!("s2 = {}", s2);

    // 文字列の足し算はformatマクロを使え
    let s = format!("{}-{}", s1, s2);
    println!("s = {}", s);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    println!("-----------------");
    // 文字列を走査する
    for c in s.chars() {
        println!("{}", c);
    }
}
