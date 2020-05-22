

fn take1(mut v1: i32) {
	v1 += 2;

	println!("{:?}", v1);
}

fn take2(v1: &mut Vec<i32>) {
	v1[1] = 33;
	take22(v1);
}

fn take22(v1: &mut Vec<i32>) {
	v1[0] = 1987;
}

fn main() {
	let v = 10i32;
	take1(v);

	let mut v = vec![1, 2];
	take2(&mut v);
	println!("{:?}", v);
}
