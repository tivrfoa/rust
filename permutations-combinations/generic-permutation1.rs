

fn permutations<T>(a: &mut Vec<T>, n: usize)
where T: std::fmt::Debug + Copy
{
	if n == 1 {
		println!("{:?}", a);
		return;
	}

	for i in 0..n {
		swap(a, i, n-1);
		permutations(a, n-1);
		swap(a, i, n-1);
	}
}

fn swap<T>(v: &mut Vec<T>, a: usize, b: usize)
where T: std::fmt::Debug + Copy
{
	let tmp = v[a];
	v[a] = v[b];
	v[b] = tmp;
}


fn main() {

	let mut v = vec![1, 2 , 3];
	let len = v.len();
	permutations(&mut v, len);

	permutations(&mut vec!["Leandro", "Santos", "Coutinho"], 3);
}
