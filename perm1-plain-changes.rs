

fn perm(a: &mut Vec<u32>, n: usize, i: usize) {

	let f = fac(n as u32);
	let mut i = 0;
	for _ in 0..f {
		dv(a);
		let tmp = a[i];
		a[i] = a[i+1];
		a[i+1] = tmp;
		i += 1;
		if i == n-1 { i = 0; }
	}
}


fn main() {
	let mut v = (0..11).collect();
	perm(&mut v, 11, 0);
	let f = fac(11);
}

fn dv(a: &mut Vec<u32>) {
	println!("{:?}", a);
}

fn fac(n: u32) -> u32 {
	let mut f = 1;
	for x in 2..=n {
		f *= x;
	}
	println!("factorial of {} is {}", n, f);
	f
}
