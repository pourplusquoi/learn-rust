impl Solution {
  pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    let mut sum = 0;
    let mut cnt = vec![0; 10];
    for d in digits.into_iter().map(|x| x as usize) {
      sum += d;
      cnt[d] += 1;
    }
    
    let mut rmd = sum % 3;
    if rmd > 0 {
      for d in (rmd..10).step_by(3) {
        if cnt[d] > 0 {
          rmd = 0;
          cnt[d] -= 1;
          break;
        }
      }
    }
    if rmd > 0 {
      let range = if rmd == 1 { [2, 5, 8] } else { [1, 4, 7] };
      for &d1 in range.iter() {
        for &d2 in range.iter() {
          if (d1 != d2 && cnt[d1] > 0 && cnt[d2] > 0) {
            rmd = 0;
            cnt[d1] -= 1;
            cnt[d2] -= 1;
            break;
          } else if (d1 == d2 && cnt[d1] > 1) {
            rmd = 0;
            cnt[d1] -= 2;
            break;
          }
        }
      }
    }
    
    let mut res = String::from("");
    if rmd > 0 {
      return res;
    }
    for i in (1..10).rev() {
      for j in 0..cnt[i] {
        res.push((i + '0' as usize) as u8 as char);
      }
    }
    if res.is_empty() {
      cnt[0] = std::cmp::min(cnt[0], 1);
    }
    for j in 0..cnt[0] {
      res.push('0');
    }
    res
  }
}
