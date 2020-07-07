

fn heaps_algorithm<T>(a: &mut Vec<T>, n: usize)
where T: std::fmt::Debug + Copy
{
	if n == 1 {
		println!("{:?}", a);
		return;
	}

	for i in 0..n-1 {
		heaps_algorithm(a, n-1);
		if n%2 == 0 {
			swap(a, n-1, i);
		} else {
			swap(a, n-1, 0);
		}
	}
	heaps_algorithm(a, n-1);
}

fn main() {

	let mut v = vec![1, 2 , 3];
	let len = v.len();
	heaps_algorithm(&mut v, len);

	heaps_algorithm(&mut vec!["Brazil", "Santos", "Earth"], 3);
}

fn swap<T>(v: &mut Vec<T>, a: usize, b: usize)
where T: std::fmt::Debug + Copy
{
	let tmp = v[a];
	v[a] = v[b];
	v[b] = tmp;
}

