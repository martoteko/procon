use proconio::{input, marker::Chars};

fn main() {
    input!(
        s: Chars,
    );

    let c_1 = s[0];
    let c_2 = s[1];
    let c_3 = s[2];

    if c_1 != c_2 {
        if c_1 != c_3 {
            println!("{}",c_1);
            return;
        }else{
            println!("{}",c_2);
            return;
        }
    } else if c_1 != c_3 {
        println!("{}",c_3);
        return;
    } else {
        for c in s {
            if c != c_1 {
                println!("{}",c);
                return;
            }
        }
    }
}
