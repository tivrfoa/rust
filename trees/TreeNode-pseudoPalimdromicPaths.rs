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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let root = root.unwrap();
		Solution::helper(Some(&root), &mut [0u32; 10])
    }

    fn helper (root: Option<&Rc<RefCell<TreeNode>>>, counts: &mut [u32]) -> i32 {
		let root = match root {
			Some(node) => node,
			None => return 0,
		};
		let node = root.borrow();
		counts[node.val as usize] += 1;
		if node.left.is_none() && node.right.is_none() {
			let count = if Solution::palindromic(&counts) { 1 } else { 0 };
			counts[node.val as usize] -= 1;
			return count;
		}
		let count = Solution::helper(node.left.as_ref(), counts) +
		            Solution::helper(node.right.as_ref(), counts);
		counts[node.val as usize] -= 1;

		count
	}

	pub fn palindromic(counts: &[u32]) -> bool {
		let mut countOdd = 0;
		for f in counts {
			if f % 2 == 1 {
				if countOdd == 0 { countOdd += 1; }
				else { return false; }
			}
		}
		true
	}
}

fn main() {
	// root = [2,3,1,3,1,null,1]
	let tree = get_node(2);
	let tree = tree.unwrap();
	{
		let mut root = tree.borrow_mut();
		root.left = get_node(3);
		root.right = get_node(1);
		let left = root.left.as_ref().unwrap();
		{ left.borrow_mut().left = get_node(3); }
		{ left.borrow_mut().right = get_node(1); }
		let right = root.right.as_ref().unwrap();
		{ right.borrow_mut().left = None; }
		{ right.borrow_mut().right = get_node(1); }
	}
	let num = Solution::pseudo_palindromic_paths(Some(tree));
	assert!(num == 2);
	println!("{}", num);
}

fn get_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
	Some(Rc::new(RefCell::new(TreeNode {
		val,
		left: None,
		right: None,
	})))
}
