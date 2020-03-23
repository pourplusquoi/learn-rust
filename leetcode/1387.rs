impl Solution {
  pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut nums = Vec::new();
    for i in lo..=hi {
      nums.push((i, Self::calc(i)));
    }
    nums.sort_by(|x, y| x.1.cmp(&y.1));
    nums[k as usize - 1].0
  }
  
  fn calc(mut num: i32) -> i32 {
    let mut res = 0;
    while num != 1 {
      res += 1;
      if num % 2 == 0 {
        num /= 2;
      } else {
        num = 3 * num + 1;
      }
    }
    res
  }
}
