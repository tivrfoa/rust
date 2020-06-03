


fn main() {
	let mut x = None;
	let y: &mut u32 = x.get_or_insert_with(|| 5);
	assert_eq!(y, &5);

	*y = 7;

	assert_eq!(x, Some(7));

	match x {
		Some(ref mut v) => *v += 10,
		None => (),
	}

	println!("{:?}", x);

	let z: &mut u32 = x.get_or_insert_with(|| 30);
	println!("{:?}", z);

	*y = 33;
	println!("{:?}", x);
}
