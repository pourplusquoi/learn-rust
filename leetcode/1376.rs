use std::cmp::max;

impl Solution {
  pub fn num_of_minutes(n: i32,
                        head_id: i32,
                        manager: Vec<i32>,
                        inform_time: Vec<i32>) -> i32 {
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
    for i in 0..n {
      let mgr = manager[i as usize];
      if mgr == -1 {
        continue;
      }
      graph[mgr as usize].push(i);
    }
    Self::dfs(&graph, &inform_time, head_id)
  }
  
  fn dfs(graph: &Vec<Vec<i32>>,
         inform_time: &Vec<i32>,
         cur: i32) -> i32 {
    let mut beneath_time = 0;
    for nxt in graph[cur as usize].iter() {
      beneath_time = max(beneath_time, Self::dfs(graph, inform_time, *nxt));
    }
    inform_time[cur as usize] + beneath_time
  }
}
