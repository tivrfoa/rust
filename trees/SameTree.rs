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

struct Solution;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() { return true; }
        if p.is_none() || q.is_none() { return false; }
        let p = Rc::try_unwrap(p.unwrap()).unwrap().into_inner();
        let q = Rc::try_unwrap(q.unwrap()).unwrap().into_inner();
        if p.val != q.val { return false; }
        
        Solution::is_same_tree(p.left, q.left) && Solution::is_same_tree(p.right, q.right)
    }
}

fn main() {
	let p = Some(Rc::new(RefCell::new(TreeNode {
		val: 1,
		left: new_node(2),
		right: new_node(3), 
	})));
	let q = Some(Rc::new(RefCell::new(TreeNode {
		val: 1,
		left: new_node(2),
		right: new_node(3), 
	})));
	assert_eq!(Solution::is_same_tree(p, q), true);
}

fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
	Some(Rc::new(RefCell::new(TreeNode {
		val,
		left: None,
		right: None,
	})))
}
