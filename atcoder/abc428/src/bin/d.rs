use proconio::input;

fn is_square(n: u128) -> Result<u128, ()> {
    if n < 2 {
        return Ok(1);
    }
    let mut left = 1;
    let mut right = n / 2;

    while left <= right {
        let mid = left + (right - left) / 2;
        let sq = mid * mid;
        if sq == n {
            return Ok(sq);
        } else if sq < n {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    Err(())
}

fn get_digits(_n: u128) -> u32 {
    let mut digits = 0;
    let mut n = _n;
    while n > 0 {
        n /= 10;
        digits += 1;
    }

    digits
}

fn main() {
    input!(
        t: i32
    );

    for _ in 0..t {
        input! {
            c: u128,
            d: u128,
        }

        let mut count = 0;
        for x in 1..(d + 1) {
            let c_x = c + x;
            let num = c * 10_u128.pow(get_digits(c_x)) as u128 + c_x;

            //このnumが平方数ならok.
            if let Ok(sq) = is_square(num) {
                count += 1;
                //println!("fn({},{}) = {} -> {}", c, c_x, num, sq);
            }
        }

        println!("{}", count);
    }
}
