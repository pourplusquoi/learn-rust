use std::collections::HashMap;

impl Solution {
  pub fn rank_teams(votes: Vec<String>) -> String {
    let n = votes[0].len();
    let mut map = HashMap::new();
    for vote in votes.iter() {
      for (i, team) in vote.chars().enumerate() {
        let vec = map.entry(team).or_insert(vec![0; n]);
        vec[i] += 1;
      }
    }
    let mut vec = Vec::new();
    for (k, v) in map.into_iter() {
      vec.push((k, v));
    }
    vec.sort_by(|lhs, rhs| {
      for (x, y) in lhs.1.iter().zip(rhs.1.iter()) {
        if x != y {
          return y.cmp(&x);
        }
      }
      return lhs.0.cmp(&rhs.0);
    });
    let mut res = String::from("");
    for (team, _) in vec.iter() {
      res += &team.to_string();
    }
    res
  }
}
