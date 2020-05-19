use std::cmp::min;
use std::time::SystemTime;

pub trait QueryInterface {
	fn increment(&mut self, i: usize, j: usize, val: i32);
	
	// why mutable reference?
	fn minimum(&mut self, i: usize, j: usize) -> i32;
}

pub struct Test;

impl Test {

	fn run_test_small(rmq: &mut dyn QueryInterface, queries: &[&[i32]]) {
		for cur_query in queries {
			if cur_query[0] == 1 {
				rmq.increment(cur_query[1] as usize, cur_query[2] as usize, cur_query[3]);
			} else {
				let value = rmq.minimum(cur_query[1] as usize, cur_query[2] as usize);
				println!("Range minimum [{}, {}] = {}",
						cur_query[1], cur_query[2], value);
			}
		}
	}

	fn run_test1(rmq: &mut dyn QueryInterface) {
		let v1: [&[i32]; 13] = [
			&[1, 0, 3, 2],
			&[1, 0, 7, 1],
			&[1, 4, 7, 10],
			&[1, 2, 5, 7],
			&[2, 0, 7],
			&[2, 1, 4],
			&[2, 2, 5],
			&[2, 0, 0],
			&[2, 1, 1],
			&[2, 2, 2],
			&[2, 3, 3],
			&[2, 4, 4],
			&[2, 5, 5],
        ];
		
		Test::run_test_small(rmq, &v1);
	}

	fn randint(i: i32, j: i32) -> i32 {
		let fi = i as f32;
		let fj = j as f32;
		let ri = ((Test::random() * (fj - fi + 1.0)) as i32) + i;
		assert!(ri >= i && ri <= j, "{} not between {} and {}", ri, i, j);

		ri
	}

	/// Returns float between 0.0 and 1.0
	fn random() -> f32 {
		let now = SystemTime::now();
		let nano = now.elapsed().unwrap().subsec_nanos();
		
		let r = ((nano%10) as f32)/10.0;
		assert!(r >= 0.0 && r <= 1.0, "{} not between 0.0 and 1.0", r);

		r
	}

	fn run_random_tests(size: usize, num_tests: usize,
			rmq1: &mut dyn QueryInterface, rmq2: &mut dyn QueryInterface)
	{
		let mut all_passed = true;

		for test_num in 1..num_tests {
			let type_ = Test::randint(1, 2);
			let mut range = [
				Test::randint(0, (size - 1) as i32),
				Test::randint(0, (size - 1) as i32)
			];
			range.sort();

			if test_num % 10000 == 0 {
				println!("Test checkpoint: {}", test_num);
			}

			if type_ == 1 {
				let val = Test::randint(1, 100);
				rmq1.increment(range[0] as usize, range[1] as usize, val);
				rmq2.increment(range[0] as usize, range[1] as usize, val);
			} else {
				let r1 = rmq1.minimum(range[0] as usize, range[1] as usize);
				let r2 = rmq2.minimum(range[0] as usize, range[1] as usize);
				if r1 != r2 {
					println!("Failure {} vs {}", r1, r2);
					all_passed = false;
				}
			}
		}

		if all_passed {
			println!("All tests passed!");
		}
	}
}

fn main() {

	/*
	Test::run_test1(&mut RangeSlow::new(8));
	println!("");
	Test::run_test1(&mut SegmentTree::new(8));
	*/

	// let size = 1_000;
	let size = 1_000_000;
	/*Test::run_random_tests(size, size, &mut RangeSlow::new(size),
			&mut SegmentTree::new(size));*/
	// Test::run_random_tests(size, size, &mut SegmentTree::new(size),
	//		&mut SegmentTree::new(size));
	Test::run_random_tests(size, size, &mut RangeSlow::new(size),
			&mut RangeSlow::new(size));

}

struct RangeSlow {
	arr: Vec<i32>,
}

impl RangeSlow {
	fn new(n: usize) -> RangeSlow {
		let arr = vec![0; n];
		Self {
			arr
		}
	}
}

impl QueryInterface for RangeSlow {

	fn increment(&mut self, i: usize, j: usize, val: i32) {
		for k in i..=j {
			self.arr[k] += val;
		}
	}

	fn minimum(&mut self, i: usize, j: usize) -> i32 {
		let mut res = self.arr[i];
		for k in i+1..=j {
			res = min(res, self.arr[k]);
		}

		res
	}
}

struct SegmentTree {
	n: usize,
	lo: Vec<usize>,
	hi: Vec<usize>,
	min: Vec<i32>,
	delta: Vec<i32>,
}

impl SegmentTree {

	fn new(n: usize) -> SegmentTree {
		let mut st = SegmentTree {
			n,
			lo: vec![0; 4*n+1],
			hi: vec![0; 4*n+1],
			min: vec![0; 4*n+1],
			delta: vec![0; 4*n+1],
		};

		st.init(1, 0, n - 1);

		st
	}

	fn init(&mut self, i: usize, a: usize, b: usize) {
		self.lo[i] = a;
		self.hi[i] = b;

		if a == b { return; }

		let m = (a+b)/2;
		self.init(2*i, a, m);
		self.init(2*i+1, m+1, b);
	}

	fn prop(&mut self, i: usize) {
		self.delta[2*i] += self.delta[i];
		self.delta[2*i+1] += self.delta[i];
		self.delta[i] = 0;
	}

	fn update(&mut self, i: usize) {
		self.min[i] = min(self.min[2*i] + self.delta[2*i],
		                  self.min[2*i+1] + self.delta[2*i+1]);
	}

	fn increment2(&mut self, i: usize, a: usize, b: usize, val: i32) {
		if b < self.lo[i] || self.hi[i] < a {
			return;
		}

		if a <= self.lo[i] && self.hi[i] <= b {
			self.delta[i] += val;
			return;
		}

		self.prop(i);

		self.increment2(2*i, a, b, val);
		self.increment2(2*i+1, a, b, val);

		self.update(i);
	}

	// CircularRMQ!
	fn minimum2(&mut self, i: usize, a: usize, b: usize) -> i32 {
		if b < self.lo[i] || self.hi[i] < a {
			return i32::MAX;
		}

		if a <= self.lo[i] && self.hi[i] <= b {
			return self.min[i] + self.delta[i];
		}

		self.prop(i);

		let min_left = self.minimum2(2*i, a, b);
		let min_right = self.minimum2(2*i+1, a, b);

		self.update(i);

		min(min_left, min_right)
	}
}

impl QueryInterface for SegmentTree {

	fn increment(&mut self, i: usize, j: usize, val: i32) {
		self.increment2(1, i, j, val);
	}

	fn minimum(&mut self, i: usize, j: usize) -> i32 {
		self.minimum2(1, i, j)
	}
}
