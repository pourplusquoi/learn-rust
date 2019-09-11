use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

impl Solution {
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        Solution::helper(&t)
    }
    
    pub fn helper(t: &Option<Rc<RefCell<TreeNode>>>) -> String {
        match t {
            None => String::from(""),
            Some(rc) => {
                let node = rc.borrow();
                let mut res = format!("{}", node.val);
                if node.right.is_some() {
                    res = format!("{}({})", res, Solution::helper(&node.left));
                    res = format!("{}({})", res, Solution::helper(&node.right));
                } else if node.left.is_some() {
                    res = format!("{}({})", res, Solution::helper(&node.left));
                }
                res
            }
        }
    }
}