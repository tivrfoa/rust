// https://www.youtube.com/watch?v=83yW2Pp6HMk
// https://atcoder.jp/contests/abc164/tasks/abc164_d

use std::io;

const MOD: usize = 2019;

pub struct Solution;

impl Solution {
	pub fn subarray_divisibility(s: & str) -> u64 {
		let n = s.len();
		let s: Vec<char> = s.chars().collect();

		let mut suf: usize = 0;
		let mut power_of_ten: usize = 1;
		let mut answer: u64 = 0;
		let mut cnt_suf: Vec<usize> = vec![0; MOD];
		cnt_suf[suf] += 1;

		for i in (0..=n-1).rev() {
			let digit: usize = s[i].to_digit(10).unwrap() as usize;
			suf = (suf + digit * power_of_ten) % MOD;
			power_of_ten = 10 * power_of_ten % MOD;
			answer += cnt_suf[suf] as u64;
			cnt_suf[suf] += 1;
		}

		answer
	}
}

fn main() {
	assert_eq!(Solution::subarray_divisibility("1817181712114"), 3);
	assert_eq!(Solution::subarray_divisibility("14282668646"), 2);
	assert_eq!(Solution::subarray_divisibility("2119"), 0);
	assert_eq!(Solution::subarray_divisibility("2019"), 2);

	let s= input();
	println!("{}", Solution::subarray_divisibility(&s));
}

fn input () -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    String::from(ret.trim())
}
