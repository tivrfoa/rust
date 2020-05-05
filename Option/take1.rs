

fn main() {

	let mut x: Option<i32> = Some(33);

	println!("{:?}", x);

	let i = x.take(); // Some(33)

	println!("{:?}", x); // None
	println!("{:?}", i); // Some(33)
	println!("{:?}", i.unwrap()); // 33
}
