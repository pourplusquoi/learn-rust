use std::cmp::max;

fn binary_search(time: &Vec<(i32, u16)>, tar: i32) -> Option<usize> {
    if time.is_empty() {
        return None;
    }
    let mut lo = 0;
    let mut hi = time.len() - 1;
    while lo < hi {
        let mid = hi - (hi - lo) / 2;
        if time[mid].0 < tar {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    if time[lo].0 >= tar {
        None
    } else {
        Some(lo)
    }
}

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let mut time: Vec<(i32, u16)> = Vec::new();
        pairs.sort_by(|lhs, rhs| lhs[1].cmp(&rhs[1]));
        for pair in pairs.iter() {
            let mut count = match binary_search(&time, pair[0]) {
                None => 1,
                Some(idx_) => time[idx_].1 + 1,
            };
            count = match time.iter().last() {
                None => count,
                Some(tuple) => max(tuple.1, count),
            };
            time.push((pair[1], count));
        }
        time.iter().last().unwrap().1 as i32
    }
}