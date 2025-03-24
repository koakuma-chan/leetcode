use std::cell::RefCell;
/// Given the root of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path
/// from the root node down to the farthest leaf node.
use std::rc::Rc;

/// Definition for a binary tree node.
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
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursion(node: Option<Rc<RefCell<TreeNode>>>, current_depth: i32) -> i32 {
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                let depth = current_depth + 1;
                let l = recursion(node.left.take(), depth);
                let r = recursion(node.right.take(), depth);
                l.max(r)
            } else {
                current_depth
            }
        }
        recursion(root, 0)
    }
}

pub fn main() {
    println!("");
}
