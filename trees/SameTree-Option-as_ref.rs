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
        Solution::is_same_tree_ref(p.as_ref(), q.as_ref())
    }
    pub fn is_same_tree_ref(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() { return true; }
        if p.is_none() || q.is_none() { return false; }
        let p = p.unwrap().borrow();
        let q = q.unwrap().borrow();
        if p.val != q.val { return false; }

		println!("{:#?}", p);
		println!("{:#?}", p.left);
		println!("{:?}", p.left.as_ref());

        Solution::is_same_tree_ref(p.left.as_ref(), q.left.as_ref()) &&
        Solution::is_same_tree_ref(p.right.as_ref(), q.right.as_ref())
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

	println!("=================================");
	let t = new_node(5);

	let tr = &t;
	match tr {
		Some(e) => {
			println!("e = {:#?}", e);
			println!("*e = {:#?}", *e);
			println!("e.borrow() = {:#?}", e.borrow());
		},
		None => println!("None {:?}", tr),
	}
	println!("=================================");

	println!("{:#?}", t);
	let t = t.unwrap();
	println!("{:#?}", t);
	let t = t.borrow();
	println!("{:#?}", t);
	println!("{:#?}", t.val);
	println!("{:#?}", t.left);
}

fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
	Some(Rc::new(RefCell::new(TreeNode {
		val,
		left: None,
		right: None,
	})))
}
