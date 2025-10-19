use proconio::input;

fn main() {
    input! {
        A: i32
    }

    println!("{}", if 400 % A == 0 { 400 / A } else { -1 });
}
