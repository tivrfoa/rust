

fn main() {

	let mut c = 0;

	let mut inc = || {
		c += 1;
		println!("incremented by 1: {}", c);
	};

	inc();
	inc();
	inc();
}
