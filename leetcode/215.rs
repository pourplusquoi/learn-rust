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

impl Solution_QuickSelect_ThreeWay {
  pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut lo = 0;
    let mut hi = nums.len();
    let mut idx = (k - 1) as usize;
    while (lo < hi) {
      let (begin, end) = Self::partition(&mut nums[lo..hi], lo);
      if begin <= idx && idx < end {
        return nums[begin];
      }
      if idx < begin {
        hi = begin;
      } else {
        lo = end;
      }
    }
    // Never reach here.
    std::i32::MIN
  }
  
  fn partition(nums: &mut [i32], offset: usize) -> (usize, usize) {
    let target = nums[0];
    let mut begin = 0;  // The begin of eq.
    let mut end = 1;    // The end of eq.
    for idx in 1..nums.len() {
      if nums[idx] == target {
        nums.swap(idx, end);
        end += 1;
      } else if nums[idx] > target {
        nums.swap(idx, begin);
        nums.swap(idx, end);
        begin += 1;
        end += 1;
      }
    }
    (begin + offset, end + offset)
  }
}
