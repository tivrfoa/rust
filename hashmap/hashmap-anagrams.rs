use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_group = HashMap::new();
        for s in strs.into_iter() {
            let mut hash_key = [0; 26];
            
            for c in s.chars() {
                hash_key[(c as u32 - 'a' as u32) as usize] += 1;
            }
            
            hash_group.entry(hash_key).or_insert(Vec::new()).push(s);
        } 
        
        hash_group.into_iter()
            .map(|(_, group)| group)
            .collect()
    }
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
	let input1 = vec_of_strings!["eat","tea","tan","ate","nat","bat"];
	println!("{:?}", Solution::group_anagrams(input1));
}
