impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut front: Vec<(i32, i32)> = Vec::new();
        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    count += 1;
                    front.push((i as i32, j as i32));
                }
            }
        }
        let neighbor = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        for step in 0.. {
            let mut new_front: Vec<(i32, i32)> = Vec::new();
            for pt in front {
                for off in &neighbor {
                    let new_i = pt.0 + off.0;
                    let new_j = pt.1 + off.1;
                    if new_i >= 0 && new_i < n as i32 &&
                       new_j >= 0 && new_j < n as i32 &&
                       grid[new_i as usize][new_j as usize] == 0 {
                        new_front.push((new_i, new_j));
                        grid[new_i as usize][new_j as usize] = 1;
                    }
                }
            }
            if new_front.is_empty() {
                return if step == 0 {-1} else {step};
            }
            front = new_front;
        }
        -1  // No land or water exists in the grid.
    }
}