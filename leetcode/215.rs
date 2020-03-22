impl Solution_QuickSelect_TwoWay {
  pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len();
    let mut idx = (k - 1) as usize;
    while (lo < hi) {
      let pivot = lo + Self::partition(&mut nums[lo..hi]);
      if pivot == idx {
        return nums[pivot];
      }
      if pivot < idx {
        lo = pivot + 1;
      } else {
        hi = pivot;
      }
    }
    // Never reach here.
    std::i32::MIN
  }
  
  fn partition(nums: &mut [i32]) -> usize {
    let target = nums[0];
    let mut lo = 0;
    let mut hi = nums.len() - 1;
    while lo < hi {
      while lo < hi && nums[hi] <= target {
        hi -= 1;
      }
      if lo < hi {
        nums[lo] = nums[hi];
      }
      while lo < hi && nums[lo] > target {
        lo += 1;
      }
      if lo < hi {
        nums[hi] = nums[lo];
      }
      nums[lo] = target;
    }
    lo
  }
}
