use std::cmp::min;

// dp[i][j] -> max num using i elements with length j.
// dp[i][j] = max(dp[i - 1][j - 1] `append` nums[i], dp[i - 1][j]),

impl Solution {
  pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k_: i32) -> Vec<i32> {
    let k = k_ as usize;
    let m = nums1.len();
    let n = nums2.len();
    let dp1 = Self::compute(&nums1, min(m, k));
    let dp2 = Self::compute(&nums2, min(n, k));
    let mut res = vec![0; k];
    for i in 0..=k {
      let j = k - i;
      if i > m || j > n {
        continue;
      }
      let cand = Self::merge(&dp1[i], &dp2[j]);
      if Self::greater(&cand, &res) {
        res = cand;
      }
    }
    res
  }
  
  fn compute(nums: &Vec<i32>, len: usize) -> Vec<Vec<i32>> {
    let mut dp = vec![Vec::new(); len + 1];
    for num in nums.iter() {
      let mut cp = dp.to_vec();
      for j in 1..=len {
        let mut tmp = dp[j - 1].to_vec();
        tmp.push(*num);
        if Self::greater(&tmp, &dp[j]) {
          cp[j] = tmp;
        }
      }
      dp = cp;
    }
    dp
  }
  
  // Self::merge is extremely ugly and takes O(N^2) time in the worst case...
  fn merge(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
    let m = nums1.len();
    let n = nums2.len();
    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::new();
    while i < m || j < n {
      if i == m {
        Self::push(nums2, &mut res, &mut j);
      } else if j == n {
        Self::push(nums1, &mut res, &mut i);
      } else {
        if nums1[i] > nums2[j] {
          Self::push(nums1, &mut res, &mut i);
        } else if nums1[i] < nums2[j] {
          Self::push(nums2, &mut res, &mut j);
        } else {
          let mut i_ = i;
          let mut j_ = j;
          while i_ < m && j_ < n && nums1[i_] == nums2[j_] {
            i_ += 1;
            j_ += 1;
          }
          if j_ >= n || (i_ < m && nums1[i_] > nums2[j_]) {
            Self::push(nums1, &mut res, &mut i);
          } else {
            Self::push(nums2, &mut res, &mut j);
          }
        }
      }
    }
    res
  }
  
  fn push(nums: &Vec<i32>, res: &mut Vec<i32>, i: &mut usize) {
    res.push(nums[*i]);
    *i += 1;
  }
  
  fn greater(lhs: &Vec<i32>, rhs: &Vec<i32>) -> bool {
    if lhs.len() != rhs.len() {
      return lhs.len() > rhs.len();
    }
    for (x, y) in lhs.iter().zip(rhs.iter()) {
      if x > y {
        return true;
      }
      if x < y {
        return false;
      }
    }
    false
  }
}
