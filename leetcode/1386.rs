impl Solution {
  pub fn max_number_of_families(n: i32, mut reserved: Vec<Vec<i32>>) -> i32 {
    reserved.sort();
    let mut res = 0;
    let mut j = 0;
    for i in 0..n {
      let mut mask = 0;
      while reserved[j][0] == i {
        mask += 1 << (reserved[j][1] as usize - 1);
        j += 1;
      }
      if mask & 30 == 0 && mask & 480 == 0 {
        res += 2;
      } else if mask & 30 == 0 || mask & 120 == 0 || mask & 480 == 0 {
        res += 1;
      }
    }
    res
  }
}
