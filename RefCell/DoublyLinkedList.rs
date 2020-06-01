use std::fmt::{
	Debug,
	Formatter,
};
use std::rc::Rc;
use std::cell::RefCell;

struct Node<Item>
{
	item: Option<Item>,
	next: Option<Rc<RefCell<Node<Item>>>>,
	prev: Option<Rc<RefCell<Node<Item>>>>,
}

impl<Item> Debug for Node<Item>
where Item: Debug {
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

impl<Item>  Node<Item> {
	pub fn new(item: Option<Item>) -> Self {
		Node {
			item,
			next: None,
			prev: None,
		}
	}
}

struct DoublyLinkedList<Item> {
	n: usize, // number of elements on list
	pre: Rc<RefCell<Node<Item>>>, // sentinel before first item
	post: Rc<RefCell<Node<Item>>>, // sentinel before last item
}

impl<Item> Debug for DoublyLinkedList<Item>
where Item: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("DoublyLinkedList")
			.field("n", &self.n)
			.field("list", &self.pre.borrow().next)
			.finish()
    }
}


impl<Item> Iterator for DoublyLinkedList<Item> {
	type Item = Item;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

impl <Item> DoublyLinkedList<Item>
where Item: Debug
{
	pub fn new() -> Self {
		let pre_ref_node = Rc::new(RefCell::new(Node::new(None)));
		let post_ref_node = Rc::new(RefCell::new(Node::new(None)));
		pre_ref_node.borrow_mut().next = Some(Rc::clone(&post_ref_node));
		post_ref_node.borrow_mut().prev = Some(Rc::clone(&pre_ref_node));
		DoublyLinkedList {
			n: 0,
			pre: pre_ref_node,
			post: post_ref_node,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.n == 0
	}

	pub fn size(&self) -> usize {
		self.n
	}

	pub fn add(&mut self, item: Item) {
		let node: Node<Item> = Node::new(Some(item));
		let value = Rc::new(RefCell::new(node));
		let last;
		{ last = self.post.borrow_mut().prev.take(); }
		{ self.post.borrow_mut().prev = Some(Rc::clone(&value)); }
		{ value.borrow_mut().next = Some(Rc::clone(&self.post)); }
		{
			let rc = Rc::clone(&value);
			let last = last.unwrap();
			last.borrow_mut().next = Some(rc);
			value.borrow_mut().prev = Some(last);
		}
		self.n += 1;
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
}

