use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for pos in chips.iter() {
            *count.entry(*pos).or_insert(0) += 1;
        }
        let mut min_cost = std::i32::MAX;
        for (key, _) in count.iter() {
            let mut cost = 0;
            for (k, v) in count.iter() {
                cost += ((key - k).abs() % 2) * v;
            }
            min_cost = min(min_cost, cost);
        }
        min_cost
    }
}