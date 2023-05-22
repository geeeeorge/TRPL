fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // ベクタの読み取り
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // ベクタを安全に読み取る
    // getのほうがよさそう、じゃないとpanicする
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // forループでの使用
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // 複数の型を入れる場合Enumを使う
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 全部同じSpreadSheetCellという型になるからok
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("aaa")),
        SpreadSheetCell::Int(100),
        SpreadSheetCell::Float(2.3),
        SpreadSheetCell::Text(String::from("bbb")),
    ];

    for i in &row {
        println!("{:?}", i);
    }

}
