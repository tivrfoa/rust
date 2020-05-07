/**
 * For this tutorial, is recommended to install the
 * follow extension:
 * $ cargo install cargo-expand
 *
 * All that matters for the input to a macro is that
 * it can be parsed. And then what matters for compilation
 * is whether the output is valid Rust, because the output
 * is what actually gets compiled.
 *
 * Basically you can think of every macro invocation as being
 * replaced by whatever its input was.
 *
 * The identifiers in the macro world are just
 * completely distinct from the variables outside of
 * macro world.
 *
 * Identifiers are used for passing names to macros.
 *
 */
#[macro_export] // allows other libraries to use it
macro_rules! avec {
	($x:ident) => {
		$x += 1;
	};

	() => {
		Vec::new()
	};

	// You can use blocks to declare variables and write
	// other expressions
	($element:expr) => {{
		let mut vs = Vec::new();
		vs.push($element);
		vs
	}};

	// If you sorround some part of your pattern
	// with dollar and then parentheses, then you can give
	// a delimiter and then either * or + or ?
	// The pattern below means one or more comma separated
	// things.
	// ? means 0 or 1
	($($elements:expr),+ $(,)?) => {{
		let mut vs = Vec::new();
		$(vs.push($elements);)+
		vs
	}};

	($element:expr; $count:expr) => {{
		let mut vs = Vec::new();
		for _ in 0..$count {
			vs.push($element);
		}
		vs
	}};
}

#[test]
fn foo() {
	let mut x = 42;
	avec!(x);
	println!("{}", x);
	assert_eq!(x, 43);
}

#[test]
fn empty_vec() {
	let x: Vec<u32> = avec![];
	assert!(x.is_empty());
}

#[test]
fn single() {
	let x: Vec<u32> = avec![42];
	assert!(!x.is_empty());
	assert_eq!(x.len(), 1);
	assert_eq!(x[0], 42);
}

#[test]
fn double() {
	let x: Vec<u32> = avec![42, 43];
	assert!(!x.is_empty());
	assert_eq!(x.len(), 2);
	assert_eq!(x[0], 42);
	assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
	let x: Vec<u32> = avec![42, 43, 1987, 33,];
	assert!(!x.is_empty());
	assert_eq!(x.len(), 4);
	assert_eq!(x[0], 42);
	assert_eq!(x[1], 43);
}

#[test]
fn test_count() {
	let x: Vec<u32> = avec![33; 10];
	assert_eq!(x.len(), 10);
	assert_eq!(x[0], 33);
	assert_eq!(x[9], 33);
}


trait MaxValue {
	fn max_value() -> Self;
}

macro_rules! max_impl {
	($t:ty) => { // ty stands for type
		impl $crate::MaxValue for $t {
			fn max_value() -> Self {
				<$t>::MAX
			}
		}
	}
}

max_impl!(u32);
max_impl!(i32);
max_impl!(u64);
max_impl!(i64);

#[test]
fn test_max_value() {
	println!("{}", u32::max_value());
	println!("{}", i32::max_value());
	println!("{}", u64::max_value());
	println!("{}", i64::max_value());

	println!("u64 / u32 = {}", u64::max_value() / u32::max_value() as u64);
	// println!("u64 / u32 = {}", u64::max_value() / u32::max_value());
}

/* does not work ...
use std::ops::Div;

type uint = u64;

impl Div for uint {
	type Output = Self;

	fn div(self, rhs: u32) -> Self::Output {
		self / rhs as u64
	}
}
*/
