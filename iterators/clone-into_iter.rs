fn main() {
    let x = vec!["Jill", "Jack", "Jane", "John"];

    let y = x
        .clone()
        .into_iter()
        .collect::<Vec<_>>();

	println!("{:?}", x);
	println!("{:?}", y);
}

