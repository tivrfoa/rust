use std::fmt::{
	Debug,
	Formatter,
};
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T>
{
	item: Option<T>,
	next: Option<Rc<RefCell<Node<T>>>>,
	prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Debug for Node<T>
where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.item.is_none() {
			return f.debug_struct("Node {}").finish();
		}
		f.debug_struct("Node")
			.field("item", &self.item)
			//.field("prev", &self.prev) // stack overflow
			.field("next", &self.next)
			.finish()
    }
}

impl<T>  Node<T> {
	pub fn new(item: Option<T>) -> Self {
		Node {
			item,
			next: None,
			prev: None,
		}
	}
}

struct DoublyLinkedList<T> {
	n: usize, // number of elements on list
	pre: Rc<RefCell<Node<T>>>, // sentinel before first item
	post: Rc<RefCell<Node<T>>>, // sentinel before last item
	current: Rc<RefCell<Node<T>>>, // used for iterator
}

impl<T> Debug for DoublyLinkedList<T>
where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("DoublyLinkedList")
			.field("n", &self.n)
			.field("list", &self.pre.borrow().next)
			.finish()
    }
}

impl <T> DoublyLinkedList<T>
where T: Debug
{
	pub fn new() -> Self {
		let pre_ref_node = Rc::new(RefCell::new(Node::new(None)));
		let current = Rc::clone(&pre_ref_node);
		let post_ref_node = Rc::new(RefCell::new(Node::new(None)));
		pre_ref_node.borrow_mut().next = Some(Rc::clone(&post_ref_node));
		post_ref_node.borrow_mut().prev = Some(Rc::clone(&pre_ref_node));
		DoublyLinkedList {
			n: 0,
			pre: pre_ref_node,
			post: post_ref_node,
			current,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.n == 0
	}

	pub fn size(&self) -> usize {
		self.n
	}

	pub fn add(&mut self, item: T) {
		let node: Node<T> = Node::new(Some(item));
		let value = Rc::new(RefCell::new(node));
		let last = self.post.borrow_mut().prev.take();
		self.post.borrow_mut().prev = Some(Rc::clone(&value));
		value.borrow_mut().next = Some(Rc::clone(&self.post));
		let last = last.unwrap();
		last.borrow_mut().next = Some(Rc::clone(&value));
		value.borrow_mut().prev = Some(last);
		
		self.n += 1;
	}

	pub fn _next(&mut self) -> Rc<RefCell<Node<T>>> {
		let tmp = &self.pre.borrow_mut().next;
		match tmp {
			Some(r) => Rc::clone(&r),
			None => Rc::new(RefCell::new(Node::new(None))),
		}
	}
}

impl<T> Iterator for DoublyLinkedList<T> {
	type Item = Rc<RefCell<Node<T>>>;

	fn next(&mut self) -> Option<Self::Item> {
		let ret;
		let mut new;
		{
			let tmp = &self.current.borrow_mut().next;
			match tmp {
				Some(r) => {
					if (r.borrow_mut().item.is_none()) {
						new = Rc::clone(&self.pre);
						ret = None;
					} else {
						new = Rc::clone(&r);
						ret = Some(Rc::clone(&r));
					}
				},
				None => {
					new = Rc::clone(&self.pre);
					ret = None;
				}
			}
		}
		std::mem::replace(&mut self.current, new);

		ret
	}
}


fn main() {

	let mut dl: DoublyLinkedList<i32> = DoublyLinkedList::new();
	println!("{:#?}", dl);
	dl.add(33);
	println!("{:#?}", dl);
	dl.add(50);
	println!("{:#?}", dl);

	// println!("{:#?}", dl.pre.borrow_mut().next);
	//
	
	let mut r1 = dl._next();
	println!("{:#?}", r1);
	r1.borrow_mut().item = Some(29);
	println!("{:#?}", r1);
	println!("{:#?}", dl);


	println!("First iteration ...");
	for r in &mut dl {
		println!("{:?}", r.borrow_mut().item);
	}

	println!("Second iteration ...");
	for r in dl {
		println!("{:?}", r.borrow_mut().item);
	}

	println!("THEN END!");
}

