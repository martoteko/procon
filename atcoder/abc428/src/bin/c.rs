use proconio::input;

fn main() {
    input!(
        q: usize
    );

    let mut last_faild_idx = 0;
    let mut waste_count = 0;
    let mut s: Vec<i32> = Vec::new();
    let mut last_sum = 0;
    for _ in 0..q {
        input!(
            n: usize
        );

        if n == 1 {
            input!(
                c: char,
            );

            if last_sum < 0 {
                //何が来てもだめ。中身は見る必要なし。waste_countインクリメント。
                waste_count += 1;
            } else {
                if c == '(' {
                    s.push(1);
                    last_sum += 1;
                } else {
                    s.push(-1);
                    last_sum -= 1;
                }
            }
        } else {
            if waste_count > 0 {
                //先に捨てるやつがある。
                waste_count -= 1;
            } else {
                last_sum -= s.pop().unwrap();
            }
        }

        if last_sum == 0 {
            println!("Yes");
        } else {
            //>0なら-1が来たらよくなるかも。
            //<0ならそれが消されるまで以降なにがきてもNo
            println!("No");
        }
    }
}
