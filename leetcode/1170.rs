use std::cmp::min;

fn f(s: &str) -> u32 {
    let mut minc = 'z';
    for c in s.chars() {
        minc = min(minc, c);
    }
    let mut res = 0;
    for c in s.chars() {
        if c == minc {
            res += 1;
        }
    }
    res
}

// The first elem which is greater than |tar|.
fn binary_search(vec: &Vec<u32>, tar: u32) -> usize {
    let mut lo = 0;
    let mut hi = vec.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if vec[mid] <= tar {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut nums: Vec<u32> = Vec::new();
        for word in words.iter() {
            nums.push(f(word));
        }
        nums.sort();
        let mut res: Vec<i32> = Vec::new();
        for query in queries.iter() {
            res.push((words.len() as i32 -
                binary_search(&nums, f(query)) as i32));
        }
        res
    }
}