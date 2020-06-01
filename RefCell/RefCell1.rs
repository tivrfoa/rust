use std::cell::RefCell;


fn main() {
	let c = RefCell::new(5);

	let five: u32 = c.into_inner();

	println!("{:?}", five);

}
