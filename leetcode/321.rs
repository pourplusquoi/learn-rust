use std::cmp::min;

// dp[i][j] -> max num using i elements with length j.
// dp[i][j] = max(dp[i - 1][j - 1] `append` nums[i], dp[i - 1][j]),

impl Solution {
  pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k_: i32) -> Vec<i32> {
    let k = k_ as usize;
    let (m, n) = (nums1.len(), nums2.len());
    let dp1 = Self::compute(&nums1, min(m, k));
    let dp2 = Self::compute(&nums2, min(n, k));
    let mut res = vec![0; k];
    for i in 0..=k {
      let j = k - i;
      if i > m || j > n {
        continue;
      }
      let cand = Self::merge(&dp1[i], &dp2[j]);
      if &cand > &res {
        res = cand;
      }
    }
    res
  }
  
  fn compute(nums: &Vec<i32>, len: usize) -> Vec<Vec<i32>> {
    let mut dp = vec![Vec::new(); 1];
    for num in nums.iter() {
      let mut cp = vec![Vec::new(); 1];
      for j in 1..=dp.len() {
        dp[j - 1].push(*num);
        if j == dp.len() || &dp[j - 1] > &dp[j] {
          cp.push(dp[j - 1].to_vec());
        } else {
          cp.push(dp[j].to_vec());
        }
      }
      dp = cp;
    }
    dp
  }
  
  // Self::merge is extremely ugly and takes O(N^2) time in the worst case...
  fn merge(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
    let (m, n) = (nums1.len(), nums2.len());
    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::new();
    while i < m || j < n {
      if i == m {
        res.append(&mut Vec::from(&nums2[j..]));
        return res;
      } else if j == n {
        res.append(&mut Vec::from(&nums1[i..]));
        return res;
      } else {
        if nums1[i] > nums2[j] {
          res.push(nums1[i]);
          i += 1;
        } else if nums1[i] < nums2[j] {
          res.push(nums2[j]);
          j += 1;
        } else {  // nums1[i] == nums2[j]
          if &nums1[i..] > &nums2[j..] {
            res.push(nums1[i]);
            i += 1;
          } else {
            res.push(nums2[j]);
            j += 1;
          }
        }
      }
    }
    res
  }
}
