fn main() {
    // スコープを抜けたので値は破棄
    {
        let s = "hello";
        println!("{}", s);
    }
    // println!("{}", s);

    // 文字列型
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // ムーブ
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);
}
