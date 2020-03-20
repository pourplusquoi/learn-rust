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
use std::rc::Rc;

struct Response {
  overall_max: i32,
  pending_max: i32,
}

impl Solution {
  pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let lhs = Self::longest(&root.as_ref().unwrap().borrow().left, true);
    let rhs = Self::longest(&root.as_ref().unwrap().borrow().right, false);
    max(max(lhs.overall_max, lhs.pending_max + 1),
        max(rhs.overall_max, rhs.pending_max + 1)) - 1
  }
  
  fn longest(node: &Option<Rc<RefCell<TreeNode>>>, turned_left: bool) -> Response {
    match *node {
      None => Response {
        overall_max: 0,
        pending_max: 0
      },
      Some(ref rc) => {
        let lhs = Self::longest(&rc.borrow().left, true);
        let rhs = Self::longest(&rc.borrow().right, false);
        let overall_max = max(max(lhs.overall_max, lhs.pending_max + 1),
                              max(rhs.overall_max, rhs.pending_max + 1));
        let pending_max = if turned_left {
          rhs.pending_max + 1
        } else {
          lhs.pending_max + 1
        };
        Response {
          overall_max: overall_max,
          pending_max: pending_max,
        }
      },
    }
  }
}
