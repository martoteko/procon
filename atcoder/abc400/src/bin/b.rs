
use proconio::input;

fn main() {
    input! {
        N: i32,
        M: i32,
    }

    let mut prev_pow: u64 = 1;
    let mut ans: u64 = 1;
    for i in 1..M+1 {
        let pow = prev_pow.checked_mul(N as u64);
        if pow.is_none() {
            println!("inf");
        }
        prev_pow = pow.unwrap();
        if ans + prev_pow > 1000000000 {
            println!("inf");
            return;
        }
        ans += prev_pow;
    }

    println!("{}", ans);
}
