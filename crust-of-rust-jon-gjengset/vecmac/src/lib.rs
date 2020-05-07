
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
