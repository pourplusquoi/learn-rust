use std::cmp::max;
use std::collections::HashMap;
use std::convert::TryInto;

impl Solution {
  pub fn find_the_longest_substring(s: String) -> i32 {
    let mut res = 0;
    let mut state = 0;
    let mut memo = HashMap::new();
    memo.insert(0, 0);
    for i in 0..s.len() {
      let c = s.chars().nth(i).unwrap();
      state ^= match "aeiou".find(c) {
        None => 0,
        Some(idx) => 1 << idx,
      };
      let j = *memo.entry(state).or_insert(i + 1);
      res = max(res, i + 1 - j);
    }
    res.try_into().unwrap()
  }
}
