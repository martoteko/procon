use proconio::{input};

fn main() {
    input!(
        x: String,
    );

    println!("{}{}", &x[0..x.len()/2], &x[x.len()/2+1..x.len()]);
}
