use std::collections::btree_map::BTreeMap;

fn main() {
	let mut count = BTreeMap::new();
	let message = "she sells sea shells by the sea shore";

	println!("{:#?}", count);
	println!("{:#?}", count.entry('a'));
	println!("{:#?}", count.entry('a').or_insert(0));
	println!("{:#?}", *count.entry('a').or_insert(0));

	for c in message.chars() {
		*count.entry(c).or_insert(0) += 1;
	}

	assert_eq!(count.get(&'s'), Some(&8));

	println!("Number of occurrences of each character in: {}", message);
	for (char, count) in &count {
		println!("{}: {}", char, count);
	}

}
