// https://codeforces.com/contest/1353/problem/D
// Constructing the Array
// Translation from neal solution

use std::collections::BinaryHeap;

use std::io;
use std::str::FromStr;
use std::error::Error;


pub struct Solution;

impl Solution {

	fn solve(n: usize) {
		let mut ans = vec![0; n];
		let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
		pq.push((n as i32, 0));

		for step in 1..n+1 {
			let current = pq.pop().unwrap();
			let start = -current.1;
			let end = start + current.0;
			let mid = (start + end - 1) / 2;
			ans[mid as usize] = step;
			pq.push((mid - start, -start));
			pq.push((end - (mid + 1), -(mid + 1)));
		}

		for i in 0..ans.len() {
			print!("{} ", ans[i]);
		}
		println!("");
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
