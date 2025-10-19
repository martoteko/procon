
use proconio::input;

fn main() {
    input! {
        N: u64,
    }

    let mut ans = 0;

    for n in 1..N/2+1 {
        let mut val = n;
        while val & 1 == 0 {
            val = val >> 1;
        }

        if val == 1 {
            ans += 1;
            continue;
        }

        let mut odd: u64 = 3;
        while odd <= val{
            let oddodd = odd*odd;
            if oddodd > val {
                break;
            }
            if val % oddodd == 0 {
                val /= oddodd;
                if val == 1 {
                    ans += 1;
                    break;
                }
                else {
                    continue;
                }
            }

            odd += 2;
        }
    }

    println!("{}", ans);
}
