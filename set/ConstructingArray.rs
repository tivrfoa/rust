use std::cmp::Ordering;
use std::cmp::max;
use std::collections::BTreeSet;

use std::io;
use std::str::FromStr;
use std::error::Error;

#[derive(Eq, Clone)]
pub struct State {
	idx: usize,
	left_zeros: usize,
	right_zeros: usize,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		let max_a = self.left_zeros + self.right_zeros;
		let max_b = other.left_zeros + other.right_zeros;
		
		if max_a == max_b {
			self.idx.cmp(&other.idx)
		} else {
			if max_a > max_b {
				Ordering::Less
			} else {
				Ordering::Greater
			}
		}
	}
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}


pub fn pop<T>(set: &mut BTreeSet<T>) -> T
where
    T: Eq + Clone + Ord,
{
    let elt = set.iter().next().cloned().unwrap();
    set.remove(&elt);
    elt
}

pub struct Solution;

impl Solution {

	fn solve(n: usize) {

		let mut ans: Vec<usize> = vec![0; n+1];

		let mut set = BTreeSet::new();

		let mut idx = n / 2;
		let mut left_zeros = n / 2;
		let mut right_zeros = n / 2;

		if n%2 == 0 {
			left_zeros -= 1;
		} else {
			idx += 1;
		}

		set.insert(State { idx, left_zeros, right_zeros });

		let mut value = 0;

		while !set.is_empty() {
			value += 1;

			let v = pop(&mut set);
			
			// println!("{:?}", v);
			ans[v.idx] = value;

			if v.left_zeros == 0 && v.right_zeros == 0 { continue; }

			if v.right_zeros > v.left_zeros {
				// right
				let i = Solution::get(v.right_zeros);
				idx = v.idx + i;
				left_zeros = idx - v.idx - 1;
				right_zeros = v.right_zeros - i;
				set.insert(State { idx, left_zeros, right_zeros });

				// left
				if v.left_zeros > 0 {
					let i = Solution::get(v.left_zeros);
					idx = v.idx - v.left_zeros + i - 1;
					right_zeros = v.idx - idx - 1;
					left_zeros = v.left_zeros - right_zeros - 1;
					set.insert(State { idx, left_zeros, right_zeros });
				}
			} else {
				// left
				let i = Solution::get(v.left_zeros);
				idx = v.idx - v.left_zeros + i - 1;
				right_zeros = v.idx - idx - 1;
				left_zeros = v.left_zeros - right_zeros - 1;
				set.insert(State { idx, left_zeros, right_zeros });

				// right
				if v.right_zeros > 0 {
					let i = Solution::get(v.right_zeros);
					idx = v.idx + i;
					left_zeros = idx - v.idx - 1;
					right_zeros = v.right_zeros - i;
					set.insert(State { idx, left_zeros, right_zeros });
				}
			}
		}

		for i in 1..ans.len() {
			print!("{} ", ans[i]);
		}
		println!("");
	}

	pub fn get(i: usize) -> usize {
		if i%2 == 0 {
			i/2
		} else {
			i/2+1
		}
	}
}


fn main() -> Result<(), Box<Error>> {

	/*for i in 1..7 {
		Solution::solve(i);
	}*/

	let T: usize = read_line()?;

	for _ in 0..T {
		let n: usize = read_line()?;
		Solution::solve(n);
	}

	Ok(())
}

fn read_line<T: FromStr>() -> Result<T, Box<Error>>
where <T as FromStr>::Err: Error + 'static
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}