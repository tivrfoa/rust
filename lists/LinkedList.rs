

#[derive(Debug)]
struct Node<T> {
	next: Option<Box<Node<T>>>,
	value: T,
}

impl<T> Node<T> {
	fn set_next(&mut self, next: Node<T>) {
		self.next = Some(Box::new(next));
	}
}

fn main() {
	let mut head = Node {
		next: None,
		value: 1,
	};

	let next = Node {
		next: None,
		value: 2,
	};

	head.set_next(next);

	println!("{:?}", head);
}
