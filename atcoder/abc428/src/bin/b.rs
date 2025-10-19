use proconio::input;

fn main() {
    input!(
        n: usize,
        k: usize,
        s: String,
    );

    let mut max_found_count = 0;
    let mut max_found_start_idx: Vec<usize> = Vec::new();

    //長さkの文字列を順番にみていく。文字数はnなので、全探索ならn-k文字まで探せばよさそう
    for start_idx in 0..(n - k + 1) {
        let mut find_count = 0;
        let seek_str = &s[start_idx..(start_idx + k)];

        for seek_idx in 0..(n - k + 1) {
            let compare_str = &s[seek_idx..(seek_idx + k)];
            if seek_str == compare_str {
                find_count += 1;
            }
        }

        if find_count > max_found_count {
            //発見回数の最大を更新！
            max_found_count = find_count;
            max_found_start_idx.clear();
            max_found_start_idx.push(start_idx);
        } else if find_count == max_found_count {
            //同率一位。ただし同じ文字の可能性あり。でも気にしない。
            max_found_start_idx.push(start_idx);
        } else {
            //発見回数が少ない
        }
    }

    //発見回数をprint
    println!("{}", max_found_count);

    //max_found_start_idxを辞書順にprint
    //まずは文字列を配列に入れる。
    let mut str_vec: Vec<String> = Vec::new();
    for idx in max_found_start_idx {
        str_vec.push(s[idx..(idx + k)].to_string());
    }
    //辞書順にソート
    str_vec.sort();
    //print
    print!("{}", str_vec[0]);
    let mut prev = &str_vec[0];
    for str in str_vec.iter().skip(1) {
        if prev != str {
            print!(" {}", str);
            prev = str;
        }
    }
    println!("");
}
