impl Solution {
  pub fn validate_binary_tree_nodes(
      n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let mut prev = Vec::new();
    for i in 0..n {
      prev.push(i);
    }
    for i in 0..n {
      let left_child = left_child[i as usize];
      let right_child = right_child[i as usize];
      let self_root = Self::find(&mut prev, i);
      if left_child > -1 {
        let left_root = Self::find(&mut prev, left_child);
        if left_root != left_child || left_root == self_root {
          return false;
        }
        prev[left_root as usize] = prev[self_root as usize];
      }
      if right_child > -1 {
        let right_root = Self::find(&mut prev, right_child);
        if right_root != right_child || right_root == self_root {
          return false;
        }
        prev[right_root as usize] = prev[self_root as usize];
      }
    }
    let mut num_root = 0;
    for (idx, num) in prev.into_iter().enumerate() {
      if idx == num as usize {
        num_root += 1;
      }
    }
    num_root == 1
  }
  
  fn find(prev: &mut Vec<i32>, mut cur: i32) -> i32 {
    while prev[cur as usize] != cur {
      prev[cur as usize] = prev[prev[cur as usize] as usize];
      cur = prev[cur as usize];
    }
    cur
  }
}
