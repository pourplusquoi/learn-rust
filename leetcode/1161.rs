use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = -10e5 as i32;
        let mut max_level = 0;
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        if let Some(node) = root {
            nodes.push(node);
        }
        for level in 1.. {
            if nodes.is_empty() {
                break;
            } else {
                let mut sum = 0;
                let mut next_nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
                for node in nodes {
                    let node_ = node.borrow();
                    sum += node_.val;
                    match node_.left {
                        None => (),
                        Some(ref left_) => {
                            next_nodes.push(left_.clone());
                        }
                    }
                    match node_.right {
                        None => (),
                        Some(ref right_) => {
                            next_nodes.push(right_.clone());
                        }
                    }
                }
                if sum > max_sum {
                    max_sum = sum;
                    max_level = level;
                }
                nodes = next_nodes;
            }
        }
        max_level
    }
}