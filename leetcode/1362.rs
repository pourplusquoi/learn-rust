impl Solution {
  pub fn closest_divisors(num: i32) -> Vec<i32> {
    let (plus1, plus2) = (num + 1, num + 2);
    let ceil = (plus2 as f64).sqrt().floor() as i32;
    for div in (1..=ceil).rev() {
      if plus1 % div == 0 {
        return vec![div, plus1 / div];
      }
      if plus2 % div == 0 {
        return vec![div, plus2 / div];
      }
    }
    unreachable!();
  }
}
