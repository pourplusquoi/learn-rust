use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_sum = 0;
        for i in 0..m {
            for j in 0..n {
                let mut visited: HashSet<usize> = HashSet::new();
                visited.insert(i * n + j);
                max_sum = max(max_sum, grid[i][j] + Solution::dfs(&grid, &mut visited, i, j));
            }
        }
        max_sum
    }
    
    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut HashSet<usize>, i: usize, j: usize) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_sum = 0;
        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (ni, nj) = (i as i32 + dir.0, j as i32 + dir.1);
            if ni < 0 || nj < 0 {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            let key = ni * n + nj;
            if ni < m && nj < n && grid[ni][nj] > 0 && !visited.contains(&key) {
                visited.insert(key);
                let cur_sum = grid[ni][nj] + Solution::dfs(grid, visited, ni, nj);
                max_sum = max(max_sum, cur_sum);
                visited.remove(&key);
            }
        } 
        max_sum
    }
}