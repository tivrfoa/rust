
macro_rules! pln {
    ($($args:expr),*) => {{
        $(
            print!("{} ", $args);
        )*
		println!();
    }}
}

macro_rules! ppln {
    ($($args:expr),*) => {{
        $(
            print!("{:#?} ", $args);
        )*
		println!();
    }}
}

macro_rules! dln {
    ($($args:expr),*) => {{
        $(
            print!("{:?} ", $args);
        )*
		println!();
    }}
}


fn main() {

	let y = 4;
	let x = &y;
	let z = Box::new(y);

	ppln!(z);

	ppln!(*x == *z);

	let a = y + *z;
    pln!(1, 2, "Hello");

}

