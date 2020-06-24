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
        Solution::is_same_tree_ref(&p, &q)
    }
    pub fn is_same_tree_ref(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q)  {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(pp), Some(qq)) => {
                let pp = pp.borrow();
                let qq = qq.borrow();
                if pp.val != qq.val { return false; }
                Solution::is_same_tree_ref(&pp.left, &qq.left) &&
                        Solution::is_same_tree_ref(&pp.right, &qq.right)
            }
        }
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
	// println!("t = {:#?}", *t); // cannot be dereferenced
	match t {
		Some(ref e) => {
			println!("e = {:#?}", e);
			println!("*e = {:#?}", *e);
			println!("e.borrow() = {:#?}", e.borrow());
		},
		None => println!("None {:?}", t),
	}

	let tr = &t; // if "ref" is not used above: ^^ value borrowed here after partial move
	match tr {
		Some(e) => {
			println!("e = {:#?}", e);
			println!("*e = {:#?}", *e);
			println!("e.borrow() = {:#?}", e.borrow());
		},
		None => println!("None {:?}", tr),
	}
	println!("tr = {:#?}", tr);
	println!("*tr = {:#?}", *tr);
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
