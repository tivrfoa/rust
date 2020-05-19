use std::collections::HashMap;

const PRIMES: [u32; 26] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
					31, 37, 41, 43, 47, 53, 59, 61, 67,
					71, 73, 79, 83, 89, 97, 101];

struct Solution {}

impl Solution {

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
		let mut ans: Vec<Vec<String>> = Vec::new();
		if strs.len() == 0 { return ans; }
		let mut map: HashMap<u32, Vec<String>> = HashMap::with_capacity(strs.len());

		for i in 0..strs.len() {
			let s = &strs[i];
			let sum = Solution::hash(s);
			match map.get_mut(&sum) {
				Some(group) => group.push(s.to_string()),
				None => {
					let mut group: Vec<String> = Vec::new();
					group.push(s.to_string());
					map.insert(sum, group);
				}
			}
			println!("{:?}", map);
		}

		for (_, value) in map { ans.push(value); }

		ans
    }

	fn hash(s: & String) -> u32 {
		let mut sum = 1;
		for c in s.chars() {
			let i = (c as u32 - 'a' as u32) as usize;
			sum *= PRIMES[i];
		}
		sum
	}
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
	let input1 = vec_of_strings!["eat","tea","tan","ate","nat","bat"];
	println!("{:?}", Solution::group_anagrams(input1));
}
