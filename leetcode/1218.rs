use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, diff: i32) -> i32 {
        let mut res = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in arr.iter() {
            let tar = num - diff;
            let &cnt = map.get(&tar)
                          .or(Some(&0))
                          .unwrap();
            let cur = map.entry(*num)
                         .or_insert(0);
            *cur = max(*cur, cnt + 1);
            res = max(res, *cur);
        }
        res
    }
}