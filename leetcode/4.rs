use std::cmp::max;
use std::cmp::min;

// Binary search, takes Θ(M+N) time and Θ(1) space.
impl Solution {
  pub fn find_median_sorted_arrays(nums1_: Vec<i32>, nums2_: Vec<i32>) -> f64 {
    let mut nums1 = &nums1_;
    let mut nums2 = &nums2_;
    if (nums1_.len() < nums2_.len()) {
      nums1 = &nums2_;
      nums2 = &nums1_;
    }
    let (m, n) = (nums1.len() as i32, nums2.len() as i32);
    let half = (m + n) / 2;
    let mut lo1 = 0;
    let mut hi1 = m;
    while lo1 < hi1 {
      let d1 = lo1 + (hi1 - lo1) / 2;
      let d2 = half - d1;
      if d2 < 0 || (d2 < n && nums1[(d1 - 1) as usize] > nums2[d2 as usize]) {
        hi1 = d1;
      } else if d2 > n || (d2 > 0 && nums2[(d2 - 1) as usize] > nums1[d1 as usize]) {
        lo1 = d1 + 1;
      } else {
        lo1 = d1;
        hi1 = d1;
      }
    }
    let lo2 = half - lo1;
    let x1 = if lo1 > 0 { nums1[(lo1 - 1) as usize] } else { std::i32::MIN };
    let y1 = if lo1 < m { nums1[lo1 as usize] } else { std::i32::MAX }; 
    let x2 = if lo2 > 0 { nums2[(lo2 - 1) as usize] } else { std::i32::MIN };
    let y2 = if lo2 < n { nums2[lo2 as usize] } else { std::i32::MAX };
    if (m + n) % 2 == 1 {
      min(y1, y2) as f64
    } else {
      (max(x1, x2) as f64 + min(y1, y2) as f64) / 2.0
    }
  }
}
