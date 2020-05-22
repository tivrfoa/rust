
fn take1(v1: &mut Vec<i32>) {
	v1[1] = 20;
	take3(v1);
	let mut v2 = vec![0,1];
	v1 = &mut v2;
}

fn take2(mut v1: &mut Vec<i32>) {
	v1[1] = 40;
	take3(&mut v1);
}

fn take3(v1: &mut Vec<i32>) {
	v1[0] = 30;
}

fn main() {
	let mut v = vec![1, 2];
	take1(&mut v);
	println!("{:?}", v);
	
	take2(&mut v);
	println!("{:?}", v);
}
