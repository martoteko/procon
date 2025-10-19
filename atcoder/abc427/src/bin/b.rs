use proconio::input;

fn main() {
    input!(
        n: i32
    );

    let mut prev: i32 = 1;

    for _i in 1..n {
        let mut cur = 0;
        let mut j = 1;

        while prev >= j {
            cur += (prev % (j*10)) / j;
            j *= 10;
        }

        prev += cur;
    }

    println!("{}", prev);
}
