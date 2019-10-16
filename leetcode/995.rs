impl Solution {
    pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut res = 0;
        let mut inc = 0;
        let mut mark = vec![0; n + 1];
        for i in 0..n {
            inc = (inc + mark[i]) % 2;
            if (a[i] + inc) % 2 == 0 {
                res += 1;
                mark[i + 1] = 1 - mark[i + 1];
                let j = i + k as usize;
                if j <= n {
                    mark[j] = 1 - mark[j];
                } else {
                    return -1;
                }
            }
        }
        res
    }
}