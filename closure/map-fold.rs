fn use_names_for_something_else(_names: Vec<&str>) {
}

fn get_len(name: &&str) -> usize {
	println!("{:?}", name.len());
	name.len()
}

fn main() {
    let names = vec!["Jane", "Jill", "Jack", "John"];
    
    let total_bytes = names
        .iter()
        // .map(|name: &&str| name.len())
        .map(get_len)
        .fold(0, |acc, len| acc + len );
        
    assert_eq!(total_bytes, 16);
	println!("{:?}", total_bytes);
    use_names_for_something_else(names);
}
