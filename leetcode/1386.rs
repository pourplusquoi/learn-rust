use std::collections::HashMap;

impl Solution_Sort {
  pub fn max_number_of_families(n: i32, mut reserved: Vec<Vec<i32>>) -> i32 {
    reserved.sort();
    let mut res = 0;
    let mut j = 0;
    for i in 0..n {
      let mut mask = 0;
      while j < reserved.len() && reserved[j][0] - 1 == i {
        mask += 1 << (reserved[j][1] - 1);
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

impl Solution_HashMap {
  pub fn max_number_of_families(n: i32, reserved: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    for seat in reserved.into_iter() {
      map.entry(seat[0]).or_insert(Vec::new()).push(seat);
    }
    let mut res = 0;
    let mut cnt = 0;
    for (_, seats) in map.into_iter() {
      let mut mask = 0;
      for seat in seats.into_iter() {
        mask += 1 << (seat[1] - 1);
      }
      if mask & 30 == 0 && mask & 480 == 0 {
        res += 2;
      } else if mask & 30 == 0 || mask & 120 == 0 || mask & 480 == 0 {
        res += 1;
      }
      cnt += 1;
    }
    res + (n - cnt) * 2
  }
}
