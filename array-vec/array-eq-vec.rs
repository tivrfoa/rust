

fn main() {
	let a = [2, 3];
	let v = vec![2, 3];

	// error[E0277]: can't compare `[{integer}; 2]` with `std::vec::Vec<{integer}>`
	// assert_eq!(a, v);
	
	assert_eq!(v, a); // ok
	
	// error[E0277]: can't compare `[{integer}; 2]` with `std::vec::Vec<{integer}>`
	//assert_eq!([2, 3], v);

	assert_eq!(v, [2, 3]); // ok


	debug_assert!(a == [12, 3]); // ok
}
