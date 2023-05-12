fn main() {
    let condition = true;
    // 右と左は同じ型である必要がある
    let n = if condition { 1 } else { 0 };

    println!("The value of number is: {}", n);

    // loop
    let mut cnt = 0;
    // loopにラベルを付けれる
    'counting_up: loop {
        println!("count = {}", cnt);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        cnt += 1;
    }
    println!("End count = {}", cnt);

    // for
    let a = [1, 2, 3, 4, 5];

    for e in a {
        println!("The value is: {}", e);
    }
    for i in 0..5 {
        println!("{}", i);
    }
}
