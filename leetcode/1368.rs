use std::collections::HashSet;
use std::collections::VecDeque;

static map: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
  pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    Self::expand(&grid, &mut queue, &mut seen, (0, 0));
    
    let mut count = 0;
    while !queue.is_empty() {
      if seen.contains(&(m - 1, n - 1)) {
        return count;
      }
      for _ in 0..queue.len() {
        let cur = queue.pop_front().unwrap();
        for direction in 0..4 {
          if direction == Self::index(&grid, &cur) {
            continue;
          }
          let delta = map[direction];
          let nxt = (cur.0 + delta.0, cur.1 + delta.1);
          Self::expand(&grid, &mut queue, &mut seen, nxt);
        }
      }
      count += 1;
    }
    count
  }
  
  fn expand(grid: &Vec<Vec<i32>>,
            queue: &mut VecDeque<(i32, i32)>,
            seen: &mut HashSet<(i32, i32)>,
            mut pt: (i32, i32)) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    while Self::validate(&pt, m, n, seen) {
      queue.push_back(pt);
      let delta = map[Self::index(grid, &pt)];
      pt = (pt.0 + delta.0, pt.1 + delta.1);
    }
  }
  
  fn validate(pt: &(i32, i32), m: i32, n: i32,
              seen: &mut HashSet<(i32, i32)>) -> bool {
    pt.0 >= 0 && pt.1 >= 0 && pt.0 < m && pt.1 < n && seen.insert(*pt)
  }
  
  fn index(grid: &Vec<Vec<i32>>, pt: &(i32, i32)) -> usize {
    grid[pt.0 as usize][pt.1 as usize] as usize - 1
  }
}
