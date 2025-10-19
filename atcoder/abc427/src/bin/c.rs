use proconio::input;
use std::{collections::VecDeque};

fn main() {
    input!(
        n: usize,
        edge_cnt: usize,

        edges: [(usize, usize); edge_cnt],
    );

    //どの閉ループも内包する点の個数が偶数ならよさそう？
    let mut graph = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u-1].push(v-1);//順方向
        graph[v-1].push(u-1);//逆方法
    }

    let mut remove_count = 0;

    //頂点の色。-1は未着色。0 or 1
    let mut color = vec![-1; n];

    for first_elem in 0..n {
        //すでに着色しているなら、判定しない（閉ループじゃない）
        if color[first_elem] == -1 {
            let mut queue = VecDeque::new();
            color[first_elem] = 0;
            queue.push_back(first_elem);

            while let Some(u) = queue.pop_front() {
                for &v in &graph[u] {
                    if color[v] == -1 {
                        color[v] = if color[u] == 1 {0} else {1};
                        queue.push_back(v);
                    } else if color[v] == color[u] {
                        //u-vを切断する。
                        remove_count += 1;
                    }
                }
            }
        }
    }

    //順方向と逆方法の２つがダブルカウントされているため、半分にする。
    println!("{}", remove_count/2);
}
