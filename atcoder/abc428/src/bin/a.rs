use proconio::input;

fn main() {
    input!(
        s: i32,
        a: i32,
        b: i32,
        x: i32,
    );

    let mut run_time = (x / (a + b)) * a;
    let rest_time = x - (x / (a + b)) * (a + b);

    if rest_time >= a {
        run_time += a;
    } else {
        run_time += rest_time;
    }

    println!("{}", s * run_time);
}
