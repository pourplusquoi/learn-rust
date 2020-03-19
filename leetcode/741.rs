use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

impl SolutionTLE {
  pub fn cherry_pickup(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    Self::dfs(&mut grid, 0, &mut res, n, (0, 0), (n - 1, n - 1));
    res
  }
  
  fn dfs(grid: &mut Vec<Vec<i32>>,
         mut count: i32, res: &mut i32, n: usize,
         cur: (usize, usize), dst: (usize, usize)) {
    let original = grid[cur.0][cur.1];
    if grid[cur.0][cur.1] == 1 {
      count += 1;
      grid[cur.0][cur.1] = 0;
    }
    
    if cur == dst {
      if dst == (0, 0) {
        *res = max(*res, count);
      } else {
        Self::dfs(grid, count, res, n, cur, (0, 0));
      }
    } else {
      let deltas =
          if dst == (0, 0) {
            [(-1, 0), (0, -1)]
          } else {
            [(1, 0), (0, 1)]
          };

      for dt in deltas.iter() {
        let nrow = cur.0 as i32 + dt.0;
        let ncol = cur.1 as i32 + dt.1;
        if nrow >= 0 && ncol >= 0
            && nrow < n as i32 && ncol < n as i32
            && grid[nrow as usize][ncol as usize] != -1 {
          let nxt = (nrow as usize, ncol as usize);
          Self::dfs(grid, count, res, n, nxt, dst);
        }
      }
    }
    grid[cur.0][cur.1] = original;
  }
}

impl SolutionTopDownUglyDP {
  pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut dp = vec![vec![vec![-2; n]; n]; n];
    dp[n - 1][n - 1][n - 1] = grid[n - 1][n - 1];
    max(0, Self::solve(&grid, &mut dp, 0, 0, 0))
  }
  
  fn solve(grid: &Vec<Vec<i32>>,
           dp: &mut Vec<Vec<Vec<i32>>>,
           r1: usize, c1: usize, r2: usize) -> i32 {
    let mut entry = dp[r1][c1][r2];
    if entry >= -1 {
      return entry;
    }
    
    let n = grid.len();
    let c2 = r1 + c1 - r2;
    
    if r1 + 1 < n && r2 + 1 < n
        && grid[r1 + 1][c1] != -1 && grid[r2 + 1][c2] != -1 {
      entry = max(entry, Self::solve(grid, dp, r1 + 1, c1, r2 + 1));
    }
    if r1 + 1 < n && c2 + 1 < n
        && grid[r1 + 1][c1] != -1 && grid[r2][c2 + 1] != -1 {
      entry = max(entry, Self::solve(grid, dp, r1 + 1, c1, r2));
    }
    if c1 + 1 < n && r2 + 1 < n
        && grid[r1][c1 + 1] != -1 && grid[r2 + 1][c2] != -1 {
      entry = max(entry, Self::solve(grid, dp, r1, c1 + 1, r2 + 1));
    }
    if c1 + 1 < n && c2 + 1 < n
        && grid[r1][c1 + 1] != -1 && grid[r2][c2 + 1] != -1 {
      entry = max(entry, Self::solve(grid, dp, r1, c1 + 1, r2));
    }
    
    if entry >= 0 {
      entry += grid[r1][c1];
      if r1 != r2 {
        entry += grid[r2][c2];
      }
    } else {
      entry = -1;
    }
    
    dp[r1][c1][r2] = entry;
    entry
  }
}

impl SolutionBottomUpDP {
  pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut dp = vec![vec![vec![-2; n]; n]; n];
    for i in 0..n {
      for j in 0..n {
        let lo = max(0, j as i32 - i as i32) as usize;
        let hi = min(n + j - i, n);
        for k in lo..hi {
          if i == 0 && j == 0 && k == 0 {
            dp[0][0][0] = grid[0][0];
            continue;
          }
          let l = i - j + k;
          let mut entry = -2;
          if i > 0 && j > 0 && grid[i - 1][k] > -1 && grid[j - 1][l] > -1 {
            entry = max(entry, dp[i - 1][j - 1][k]);
          }
          if i > 0 && l > 0 && grid[i - 1][k] > -1 && grid[j][l - 1] > -1 {
            entry = max(entry, dp[i - 1][j][k]);
          }
          if k > 0 && j > 0 && grid[i][k - 1] > -1 && grid[j - 1][l] > -1 {
            entry = max(entry, dp[i][j - 1][k - 1]);
          }
          if k > 0 && l > 0 && grid[i][k - 1] > -1 && grid[j][l - 1] > -1 {
            entry = max(entry, dp[i][j][k - 1]);
          }
          if entry >= 0 {
            entry += grid[i][k];
            if i != j {
              entry += grid[j][l];
            }
          } else {
            entry = -1;
          }
          dp[i][j][k] = entry;
        }
      }
    }
    max(0, dp[n - 1][n - 1][n - 1])
  }
}
