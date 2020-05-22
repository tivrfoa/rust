use std::fmt::Write;

pub struct Solution;

impl Solution {
	pub fn largest_number(mut nums: Vec<i32>) -> String {
		if nums.len() == 0 { return String::from(""); }
		let len = nums.len();
		Solution::sort(&mut nums, 0, len - 1);
		//let ans: String = nums.into_iter().map(|i| i.to_string())
		//		.collect();
		//let ans: String = nums.iter().map(ToString::to_string)
		//		.collect();
		let mut ans = String::new();
		for n in nums {
			if ans.len() != 0 || n != 0 {
				let res = write!(&mut ans, "{}", n);
				match res {
					Ok(()) => (),
					Err(e) => println!("something bad: {:?}", e)
				}
			}
		}

		if ans.len() == 0 { ans = String::from("0"); }
		
		ans
	}

	pub fn compare(s1: &str, s2: &str) -> i32 {
		if s1 == s2 { return 0; }

		if s1.starts_with(s2) {
			return Solution::compare(&s1[s2.len()..], s2);
		}

		if s2.starts_with(s1) {
			return Solution::compare(s1, &s2[s1.len()..]);
		}

		let mut index = 0;
		while index < s1.len() && index < s2.len() {
			let ch1 = &s1[index..index+1];
			let ch2 = &s2[index..index+1];
			if ch1 > ch2 {
				return 1;
			} else if ch1 < ch2 {
				return -1;
			}
			index += 1;
		}

		0
	}

	pub fn sort(nums: &mut Vec<i32>, start: usize, end: usize) {
		if start < end {
			let pi = Solution::partition(nums, start, end);

			if pi > 0 {
				Solution::sort(nums, start, pi - 1);
			}
			Solution::sort(nums, pi + 1, end);
		}
	}

	pub fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
		let pivot = &nums[end].to_string();
		let mut i = start;
		for j in start..end {
			if Solution::compare(&nums[j].to_string(), pivot) == 1 {
				let tmp = nums[i];
				nums[i] = nums[j];
				nums[j] = tmp;
				i += 1;
			}
		}

		let tmp = nums[i];
		nums[i] = nums[end];
		nums[end] = tmp;

		i
	}
}


fn main() {
	let v1 = vec![3,30,34,5,9];
	let v2 = vec![10,2];
	let v3 = vec![0,0];
	let v4 = vec![824,938,1399,5607,6973,5703,9609,4398,8247];
	let v5 = vec![121,12];

	assert!(Solution::largest_number(v1) == "9534330");
	assert!(Solution::largest_number(v2) == "210");
	assert!(Solution::largest_number(v3) == "0");
	assert!(Solution::largest_number(v4) == "9609938824824769735703560743981399");
	assert!(Solution::largest_number(v5) == "12121");
}
