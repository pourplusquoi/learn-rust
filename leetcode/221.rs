use std::cmp::max;
use std::cmp::min;

impl Solution {
  pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = if (matrix.is_empty()) { 0 } else { matrix[0].len() };
    let mut max_sqr = 0;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
      for j in 0..n {
        dp[i + 1][j + 1] = if matrix[i][j] == '0' {
          0
        } else {
          let mut sqr = min(dp[i][j + 1], dp[i + 1][j]) as usize;
          if matrix[i - sqr][j - sqr] == '1' {
            sqr += 1
          }
          sqr as i32
        };
        max_sqr = max(max_sqr, dp[i + 1][j + 1]);
      }
    }
    max_sqr * max_sqr
  }
}
