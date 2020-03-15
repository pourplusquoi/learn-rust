impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut incr: Vec<i32> = Vec::new();
        for &num in nums.iter() {
            if incr.is_empty() || *incr.iter().last().unwrap() < num {
                incr.push(num);
            } else {
                let idx = Self::bisect(&incr, num);
                incr[idx] = num;
            }
        }
        incr.len() as i32
    }
    
    fn bisect(incr: &Vec<i32>, num: i32) -> usize {
        let mut lo = 0;
        let mut hi = incr.len() - 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if incr[mid] >= num {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}