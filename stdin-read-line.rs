// https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/15
use std::io;
use std::str::FromStr;
use std::error::Error;

fn read_line<T: FromStr>() -> Result<T, Box<Error>>
where <T as FromStr>::Err: Error + 'static
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

fn main() -> Result<(), Box<Error>> {
    let u: u32 = read_line()?;
    let v: String = read_line()?;
    println!("{}", u);

	let v: Vec<i32> = "1 2 3 4 5 6\n"
		.trim()
		.split(" ")
		.map(|val| val.parse().unwrap())
		.collect();
    println!("{:?}", v);

	let mut v = read_int_array();
	println!("Your int array: {:?}", v);

	// sorting ...
	v.sort();

	// destructuring with tuples
	let (a, b, c) = (v[0], v[1], v[2]);
	println!("{} {} {}", a, b, c);


	let mut v: Vec<u32> = read_array();
    println!("{:?}", v);

    Ok(())
}

fn read_int_array() -> Vec<i32> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let v: Vec<i32> = input
		.trim()
		.split(" ")
		.map(|val| val.parse().unwrap())
		.collect();

	v
}

fn read_array<T: FromStr>() -> Vec<T>
		where <T as std::str::FromStr>::Err: std::fmt::Debug {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let v: Vec<T> = input
		.trim()
		.split(" ")
		.map(|val| val.parse().unwrap())
		.collect();

	v
}
