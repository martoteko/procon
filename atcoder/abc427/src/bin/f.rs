use proconio::input;

fn solve(N: usize, A: Vec<i32>, M: i32) -> i32 {
    let mut count = 1;

    for i in n+2..N {
        let b = A[i];

        if (a + b) % M == 0 {
            count += 1;            
        }

        //３このケース

        for j in i+2..N {
            let c = A[j];

            if (a + b + c) % M == 0 {
                count += 1;            
            }

            //4このケース...

            
        }
    }

    count
}

fn main() {
    input!(
        N: usize,
        M: i32,
        A: [i32; N],
    );

    let mut count = 1;

    for n in 0..N {
        let a = A[n];

        //2このケース

        for i in n+2..N {
            let b = A[i];

            if (a + b) % M == 0 {
                count += 1;            
            }

            //３このケース

            for j in i+2..N {
                let c = A[j];

                if (a + b + c) % M == 0 {
                    count += 1;            
                }

                //4このケース...

                
            }
        }
    }

}
