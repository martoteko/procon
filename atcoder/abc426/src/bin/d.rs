use proconio::{input, marker::Chars};

fn main() {
    input!(
        t: i32,
        pairs: [(i32, Chars); t],
    );
    for (n, s) in pairs {

        let mid_idx = (n / 2) as usize;
        let mut ans = vec![0, 0];
        let mut flip_left = vec![false, false];
        let mut flip_right = vec![false, false];

        if s[mid_idx] == '0' {
            ans[0] += 0;
            ans[1] += 1;
            flip_right[1] = true;
        } else {
            ans[0] += 1;
            ans[1] += 0;
            flip_right[0] = true;
        }

        //println!("[DBG] mid_idx={} ans={:?} flip_left={:?} flip_right={:?}", 
        //mid_idx, ans, flip_left, flip_right);

        //偶数の場合、-1した値は奇数になり、それまでの回数に加えて、今回の片側だけ処理する。
        //nまでの奇数を処理する。

        for i in 1..n/2+1 {
            // ..... [mid_idx-1] [mid_idx] [mid_idx+1] ..... 

            //両端の２文字を揃える回数を加算する。ただし偶数ならrightはない。
            let left = s[(mid_idx - i as usize) as usize];
            if left == '0' {
                ans[1] += 1;   
                ans[0] += if flip_left[0] { 2 } else { 0 };
                flip_left[1] = true;
            } else {
                ans[0] += 1;  
                ans[1] += if flip_left[1] { 2 } else { 0 };
                flip_left[0] = true;
            }

            if n % 2 == 1 {
                let right = s[(mid_idx + i as usize) as usize];
                if right == '0' {
                    ans[1] += 1;   
                    ans[0] += if flip_right[0] { 2 } else { 0 }; 
                    flip_right[1] = true;
                } else {
                    ans[0] += 1;  
                    ans[1] += if flip_right[1] { 2 } else { 0 }; 
                    flip_right[0] = true;
                }
            }

            // println!("[DBG] i={} ans={:?} flip_left={:?} flip_right={:?}", 
            // i, ans, flip_left, flip_right);

        }

        println!("{}", ans[0].min(ans[1]));
    }
}
