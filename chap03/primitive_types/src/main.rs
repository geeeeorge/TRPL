fn main() {
    // 同じ型同士にする
    let x: usize = 6;
    let y: f64 = 1.5;
    let z = (x as f64) / y;
    println!("z: {}", z); // 4

    // 整数同士の除算は切り捨て
    let x = 3;
    let y = 2;
    // printの中で計算する際はこのように
    println!("{}", x / y); // 1

    // tuple
    let t = (100, 0.1, 1);
    // 要素にアクセス
    let t0 = t.0;
    let t1 = t.1;
    let t2 = t.2;
    println!("{}, {}, {}", t0, t1, t2);

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // コンパイル時に長さ固定
    // 初期化
    let b = [0; 5]; // [0, 0, 0, 0, 0]

    let first = a[0];
    let second = a[1];
    let last = a[a.len() -1]; // -1はダメだった
    println!("{}, {}, {}", first, second, last);
}
