// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::cmp::max;
use std::cmp::min;
use std::rc::Rc;

enum Response {
  Valid {
    total: i32,
    max_sum: i32,
    min_val: i32,
    max_val: i32,
  },
  Invalid {
    max_sum: i32,
  },
}

impl Response {
  pub fn is_valid(&self) -> bool {
    !self.is_invalid()
  }
  
  pub fn is_invalid(&self) -> bool {
    match *self {
      Response::Invalid { max_sum } => true,
      _ => false,
    }
  }
  
  pub fn max_sum(&self) -> i32 {
    match *self {
      Response::Valid {
        total,
        max_sum,
        min_val,
        max_val
      } => max_sum,
      Response::Invalid { max_sum } => max_sum, 
    }
  }
  
  pub fn total(&self) -> Option<i32> {
    match *self {
      Response::Valid {
        total,
        max_sum,
        min_val,
        max_val
      } => Some(total),
      Response::Invalid { max_sum } => None,
    }
  }
    
  pub fn min_val(&self) -> Option<i32> {
    match *self {
      Response::Valid {
        total,
        max_sum,
        min_val,
        max_val
      } => Some(min_val),
      Response::Invalid { max_sum } => None,
    }
  }
  
  pub fn max_val(&self) -> Option<i32> {
    match *self {
      Response::Valid {
        total,
        max_sum,
        min_val,
        max_val
      } => Some(max_val),
      Response::Invalid { max_sum } => None,
    }
  }
}

impl Solution {
  pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    Self::traverse(&root).max_sum()
  }
  
  fn traverse(node: &Option<Rc<RefCell<TreeNode>>>) -> Response {
    match *node {
      None => Response::Valid {
        total: 0,
        max_sum: 0,
        min_val: std::i32::MAX,
        max_val: std::i32::MIN,
      },
      Some(ref rc) => {
        let rfc = rc.borrow();
        let lhs = Self::traverse(&rfc.left);
        let rhs = Self::traverse(&rfc.right);
        if Self::is_valid(&rfc, &lhs, &rhs) {
          let total = rfc.val + lhs.total().unwrap() + rhs.total().unwrap();
          Response::Valid {
            total: total,
            max_sum: max(total, max(lhs.max_sum(), rhs.max_sum())),
            min_val: min(rfc.val, lhs.min_val().unwrap()),
            max_val: max(rfc.val, rhs.max_val().unwrap()),
          }
        } else {
          Response::Invalid {
            max_sum: max(lhs.max_sum(), rhs.max_sum()),
          }
        }
      }
    }
  }
  
  fn is_valid(node: &TreeNode, lhs: &Response, rhs: &Response) -> bool {
    lhs.is_valid() && rhs.is_valid()
        && (node.left.is_none() || lhs.max_val().unwrap() < node.val)
        && (node.right.is_none() || rhs.min_val().unwrap() > node.val)
  }
}
