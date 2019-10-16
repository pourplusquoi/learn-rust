use std::collections::VecDeque;

fn modulo(n: i32) -> i32 {
    let num = 1_000_000_007;
    if n < 0 {
        n + num
    } else {
        n % num
    }
}

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let mut dp: Vec<VecDeque<i32>> = Vec::new();
        let mut ps: Vec<i32> = vec![1; 6];
        let mut sum = 6;
        for i in 0..6 {
            let mut entry = vec![0; roll_max[i] as usize];
            entry[0] = 1;
            dp.push(entry.into_iter().collect());
        }
        for _ in 1..n {
            let mut ns = sum;
            for i in 0..6 {
                dp[i].push_front(modulo(sum - ps[i]));
                let delta = modulo(dp[i].front().unwrap() - dp[i].back().unwrap());
                ps[i] = modulo(ps[i] + delta);
                ns = modulo(ns + delta);
                dp[i].pop_back();
            }
            sum = ns;
        }
        sum
    }
}