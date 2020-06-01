fn run<F>(f: F)
where F: Fn() {
	f()
}

fn add3<F>(f: F) -> i32
where F: Fn(i32) -> i32 {
	f(3)
}

struct A<F: Fn(i32) -> i32> {
	f: F,
}

fn main() {
	let p = || println!("Hello from run function!");
	run(p);

	let x = |i| i * 10;

	let a = A {
		f: x,
	};

	println!("{}", add3(a.f));
	println!("{}", x(10));
}
