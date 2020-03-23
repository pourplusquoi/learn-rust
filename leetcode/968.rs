use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

impl SolutionDP {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let res = Self::solve(&root);
        return min(res[1], res[2]);
    }
    
    fn solve(node: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 3] {
        match node {
            &None => [0, 0, 9999],
            &Some(ref rc) => {
                let lhs = Self::solve(&rc.borrow().left);
                let rhs = Self::solve(&rc.borrow().right);

                let lm = min(lhs[1], lhs[2]);
                let rm = min(rhs[1], rhs[2]);

                let v0 = lhs[1] + rhs[1];
                let v1 = min(lhs[2] + rm, lm + rhs[2]);
                let v2 = 1 + min(lhs[0], lm) + min(rhs[0], rm);
                [v0, v1, v2]
            },
        }
    }
}

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type WrapNode = Option<Rc<RefCell<TreeNode>>>;

impl SolutionGreedy {
    pub fn min_camera_cover(root: WrapNode) -> i32 {
        let mut res = 0;
        let mut covered: HashSet<*const WrapNode> = HashSet::new();
        Self::dfs(&root, &None, &mut covered, &mut res);
        res
    }
    
    fn dfs(cur: &WrapNode, par: &WrapNode,
           covered: &mut HashSet<*const WrapNode>,
           res: &mut i32) {
        match cur {
            &None => (),
            &Some(ref rc) => {
                let left = &rc.borrow().left;
                let right = &rc.borrow().right;
                Self::dfs(left, cur, covered, res);
                Self::dfs(right, cur, covered, res);
                if (par.is_none() && !Self::contains(covered, cur))
                    || !Self::contains(covered, left)
                    || !Self::contains(covered, right) {
                    *res += 1;
                    covered.insert(par as *const WrapNode);
                    covered.insert(cur as *const WrapNode);
                    covered.insert(left as *const WrapNode);
                    covered.insert(right as *const WrapNode);
                }
            }
        }
    }
    
    fn contains(covered: &HashSet<*const WrapNode>, node: &WrapNode) -> bool {
        match node {
            &None => true,
            &Some(_) => covered.contains(&(node as *const WrapNode)),
        }
    }
}