struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans: Vec<Vec<i32>> = Vec::new();
		ans.push(Vec::new());

		for n in nums {
			let mut new_subsets = Vec::new();
			for curr in &ans {
				let mut tmp: Vec<i32> = Vec::with_capacity(curr.len() + 1);
				for x in curr { tmp.push(*x); }
				tmp.push(n);
				new_subsets.push(tmp);
			}
			for curr in new_subsets { ans.push(curr); }
		}

		ans
    }
}

fn main() {
	let v = vec![1, 2, 3];
	println!("{:?}", Solution::subsets(v));
}
