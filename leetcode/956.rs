use std::cmp::max;

impl Solution {
  pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
    let sum = rods.iter().sum::<i32>() as usize;
    let n = rods.len();
    let m = sum * 2 + 1;
    let mut dp = vec![vec![std::i32::MIN; m]; n + 1];
    dp[0][sum] = 0;
    for i in 0..n {
      for s in 0..m {
        let mut res = dp[i][s];
        if s >= rods[i] as usize {
          res = max(res, rods[i] + dp[i][s - rods[i] as usize]);
        }
        if s < m - rods[i] as usize {
          res = max(res, dp[i][s + rods[i] as usize]);
        }
        dp[i + 1][s] = res;
      }
    }
    dp[n][sum]
  }
}
