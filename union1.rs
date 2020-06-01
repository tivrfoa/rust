
// #[derive(Debug)]
#[repr(C)]
union SDL {
	t: i32,
	padding: [char; 7],
}

fn main() {

	let u = SDL { t: 33 };

	println!("{:?}", u);

}
