use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut freqs: HashMap<i32, u16> = HashMap::new();
        for num in nums.iter() {
            *freqs.entry(*num).or_insert(0) += 1;
        }
        if freqs.len() == 1 || freqs.len() == nums.len() {
            return idx as i32;
        }

        let mut counts: HashMap<u16, u16> = HashMap::new();
        for (_, freq) in freqs.iter() {
            *counts.entry(*freq).or_insert(0) += 1;
        }

        for idx in (0..nums.len()).rev() {
            if counts.len() == 2 {
                let curlen = idx as i32 + 1;
                if counts.get(&1) == Some(&1) {
                    return curlen;
                }
                let mut keys = counts.keys().collect::<Vec<_>>();
                keys.sort();
                if keys[1] - keys[0] == 1 && counts.get(keys[1]) == Some(&1) {
                    return curlen;
                }
            }

            let num = &nums[idx];
            let freq = freqs.get_mut(num).unwrap();
            let count = counts.entry(*freq).or_default();

            *count -= 1;
            if *count == 0 {
                counts.remove(freq);
            }

            *freq -= 1;
            if *freq == 0 {
                freqs.remove(num);
            } else {
                *counts.entry(*freq).or_insert(0) += 1;
            }
        }
        0
    }
}