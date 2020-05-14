use std::collections::BinaryHeap;


fn main() {


	let mut bh = BinaryHeap::new();

	bh.push((45, 3));
	bh.push((20, 23));
	bh.push((20, 30));
	bh.push((120, 30));

	println!("{:?}", bh);
}
