use std::cmp::max;
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
