pub struct Solution;

impl Solution {
	pub fn my_atoi(s: String) -> i32 {
		let s = s.trim_start();

		if s.is_empty() { return 0; }

		let max_integer_len = 10;
		
		let mut first = usize::MAX;
		let mut last = usize::MAX;
		let mut p = 0;
		let mut chars = s.chars();
		let mut c = chars.next().unwrap();
		let is_negative = c == '-';
		if c == '-' || c == '+' {
			p = 1;
			c = match chars.next() {
				Some(c) => c,
				None => return 0,
			}
		}

		for i in p..s.len() {
			if first == usize::MAX {
				if c < '0' || c > '9' { return 0; }
				if c > '0' { first = i; }
			} else {
				if c < '0' || c > '9' {
					break;
				}
				last = i;
				if last - first + 1 > max_integer_len { break; }
			}
			c = match chars.next() {
				Some(c) => c,
				None => '0',
			}
		}

		if first == usize::MAX { return 0; }
		if last == usize::MAX { last = first; }

		let i = first as usize;
		let j = last as usize;

		let n_len = last - first + 1;
		if n_len < max_integer_len {
			let n: i32 = s[i..j + 1].parse().unwrap();
			if is_negative { return n * -1; }
			return n;
		} else if n_len > max_integer_len {
			if is_negative { return i32::min_value(); }
			return i32::max_value();
		} else {
			let mut n: i64 = s[i..j + 1].parse().unwrap();
			if is_negative { n *= -1; }
			if n < i32::MIN as i64 { return i32::min_value(); }
			if n > i32::MAX as i64 { return i32::max_value(); }
			return n as i32;
		}
	}
}

fn main() {

	println!("{}", Solution::my_atoi(String::from("+")));
	println!("{}", Solution::my_atoi(String::from("Leandro")));
	println!("{}", Solution::my_atoi(String::from("42")));
	println!("{}", Solution::my_atoi(String::from("+42")));
	println!("{}", Solution::my_atoi(String::from("   -42")));
	println!("{}", Solution::my_atoi(String::from("4193 with words")));
}
