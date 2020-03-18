use std::cmp::Ordering;
use std::cmp::max;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Engineer {
  spd: i32,
  eff: i32,
}

impl Engineer {
  pub fn new(spd: i32, eff: i32) -> Self {
    Engineer {
      spd: spd,
      eff: eff,
    }
  }
}

impl Ord for Engineer {
    fn cmp(&self, other: &Engineer) -> Ordering {
        other.spd.cmp(&self.spd)
    }
}

impl PartialOrd for Engineer {
    fn partial_cmp(&self, other: &Engineer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
  pub fn max_performance(n: i32, spd: Vec<i32>, eff: Vec<i32>, k: i32) -> i32 {
    let modulo = 1_000_000_007;
    let mut engs: Vec<Engineer> = Vec::new();
    for i in (0..n).map(|x| x as usize) {
      engs.push(Engineer::new(spd[i], eff[i]));
    }
    engs.sort_by(|x, y| y.eff.cmp(&x.eff));
    let mut max_perf: u64 = 0;
    let mut sum_spd: u64 = 0;
    let mut heap = BinaryHeap::new();
    for i in (0..n).map(|x| x as usize) {
      heap.push(engs[i]);
      sum_spd += engs[i].spd as u64;
      if heap.len() > k as usize {
        sum_spd -= heap.pop().unwrap().spd as u64;
      }
      max_perf = max(max_perf, sum_spd * engs[i].eff as u64);
    }
    (max_perf % modulo) as i32
  }
}
