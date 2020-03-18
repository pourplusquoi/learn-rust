use std::cmp::max;

impl Solution {
  pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut cur_max = 0;
    for i in 0..light.len() {
      cur_max = max(cur_max, light[i]);
      if cur_max == (i + 1) as i32 {
        res += 1;
      }
    }
    res
  }
}
