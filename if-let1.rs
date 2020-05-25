

fn test1(num: Option<u32>) {
	if let Some(n) = num {
		println!("Testing if let ... {}", n);
		println!("{:#?}", n);
	} else {
		println!("else of if let ...");
	}

	println!("{:#?}", num);
	// println!("{}", n); // cannot find value `n` in this scope
}

fn main() {

	let n1: Option<u32> = Some(33);

	println!("{:#?}", n1);

	test1(Some(2020));
	test1(None);
}
