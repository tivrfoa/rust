use std::time::{Duration};
use std::thread::sleep;

fn f1() {
	// let v = Box::new(vec![[10; 1_000_000]; 1_000]); // stack overflow
	let mut v: Vec<Vec<i32>> = Vec::new();
	for i in 0..1_000_000 {
		let mut vv = Vec::with_capacity(1_000);
		for j in 0..1_000 {
			vv.push(j);
		}
		v.push(vv);
	}
	println!("{}", v[999_999][100]);
}

fn f2() {
	// let v = Box::new(vec![[10; 1_000_000]; 1_000]); // stack overflow
	let mut v: Vec<Vec<i32>> = Vec::new();
	for i in 0..1_000_000 {
		let mut vv = Vec::with_capacity(1_000);
		for j in 0..1_000 {
			vv.push(j);
		}
		v.push(vv);
	}
	println!("{}", v[999_999][100]);
}

fn main() {
	
	sleep(Duration::new(2, 0));

	f1();

	sleep(Duration::new(5, 0));

	f2();

	sleep(Duration::new(2, 0));
}
