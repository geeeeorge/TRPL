use std::collections::HashMap;

fn main() {
    // 連想配列作成2種
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Red"), 1);
    scores1.insert(String::from("Blue"), 2);
    println!("scores1 = {:?}", scores1);

    let teams = vec![String::from("Red"), String::from("Blue")];
    let initial_scores = vec![3, 4];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores2 = {:?}", scores2);

    // 所有権がらみの検証
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 参照ならいけるが、ライフタイムが重要になる
    // map.insert(&field_name, &field_value);
    println!("map = {:?}", map);
    // field_name, field_valueはmapにムーブされた
    // println!("field_name = {}", field_name);
}
