use proconio::{input, marker::Chars};

use std::collections::BTreeMap;

fn main() {
    input!(
        n: i32,
        q: i32,
        pairs: [(i32, i32); q],
    );

    let mut v_min = 0;

    let mut map = BTreeMap::new();

    for (x, y) in pairs {
        if v_min == 0 {
            println!("{}", x);
            v_min = x+1;
            map.insert(y, x);
            continue;
        }

        if x < v_min {
            println!("0");
            continue;
        }

        //v_min以上しかない。v_min==xならxだけ更新するので、最低１つ分。
        let mut count = x - v_min + 1;

        //v_min ~ xの間には更新後にその値になったものがあるので、個数を足す。
        for (ver, cnt) in &map {
            if *ver > x {
                break;
            }

            count += cnt;
        }

        println!("{}",count);
        v_min = x + 1;

        //x以下のmapキーは削除
        let rem: Vec<i32> = map.range(..=x).map(|(k, _)| *k).collect();
        for k in rem {
            map.remove(&k);
        }

        //yのmapがあるなら、countを足す。なければ新規に追加。
        let entry = map.entry(y).and_modify(|curr| *curr += count).or_insert(count);
    }
}
