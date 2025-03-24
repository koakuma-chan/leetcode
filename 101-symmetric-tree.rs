use std::cell::RefCell;
/// Given the root of a binary tree, check whether it is a mirror of itself
/// (i.e., symmetric around its center).
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let mut root_borrow_mut = root.borrow_mut();
        let mut stack = vec![(root_borrow_mut.left.take(), root_borrow_mut.right.take())];
        while let Some(pair) = stack.pop() {
            match pair {
                (Some(left), Some(right)) => {
                    let mut left_borrow_mut = left.borrow_mut();
                    let mut right_borrow_mut = right.borrow_mut();
                    if left_borrow_mut.val != right_borrow_mut.val {
                        return false;
                    }
                    stack.push((left_borrow_mut.left.take(), right_borrow_mut.right.take()));
                    stack.push((left_borrow_mut.right.take(), right_borrow_mut.left.take()));
                }
                (None, None) => continue,
                _ => return false,
            }
        }
        true
    }
}

pub fn main() {
    println!("");
}
