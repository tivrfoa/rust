

fn main() {

	let mut x = 8u8;
	let mut y = &mut x;

	// cannot borrow `x` as immutable because it is also borrowed as mutable
	// println!("{:?} {:?}", x, y);
	println!("{:?}", y);

	*y = 10;

	y = &mut 0u8;
	println!("{:?}", x);
}
