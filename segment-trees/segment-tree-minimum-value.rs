use std::cmp::min;
use std::time::SystemTime;

pub trait QueryInterface {
	fn increment(&mut self, i: usize, j: usize, val: i32);
	fn minimum(&self, i: usize, j: usize) -> i32;
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
		((Test::random() * (fj - fi + 1.0)) as i32) + i
	}

	fn random() -> f32 {
		let now = SystemTime::now();
		let nano = now.elapsed().unwrap().subsec_nanos();
		
		((nano%10) as f32)/10.0
	}

	fn run_random_tests(size: i32, num_tests: i32,
			rmq1: &mut dyn QueryInterface, rmq2: &mut dyn QueryInterface)
	{
		let mut all_passed = true;

		for test_num in 1..num_tests {
			let type_ = Test::randint(1, 2);
			let mut range = [Test::randint(0, size-1), Test::randint(0, size-1)];
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
				let r2 = rmq1.minimum(range[0] as usize, range[1] as usize);
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

	Test::run_test1(&mut RangeSlow::new(8));

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

	fn minimum(&self, i: usize, j: usize) -> i32 {
		let mut res = self.arr[i];
		for k in i+1..=j {
			res = min(res, self.arr[k]);
		}

		res
	}
}

struct SegmentTree {
	n: usize,
	lo: Vec<i32>,
	hi: Vec<i32>,
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

		st.init(1, 0, (n-1) as i32);

		st
	}

	fn init(&mut self, i: usize, a: i32, b: i32) {
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

	fn increment(&mut self, i: usize, a: i32, b: i32, val: i32) {
		if b < self.lo[i] || self.hi[i] < a {
			return;
		}

		if a <= self.lo[i] && self.hi[i] <= b {
			self.delta[i] += val;
			return;
		}

		self.prop(i);

		self.increment(2*i, a, b, val);
		self.increment(2*i+1, a, b, val);

		self.update(i);
	}

	// CircularRMQ!
	fn minimum(&mut self, i: usize, a: i32, b: i32) -> i32 {
		if b < self.lo[i] || self.hi[i] < a {
			return i32::MAX;
		}

		if a <= self.lo[i] && self.hi[i] <= b {
			return self.min[i] + self.delta[i];
		}

		self.prop(i);

		let min_left = self.minimum(2*i, a, b);
		let min_right = self.minimum(2*i+1, a, b);

		self.update(i);

		min(min_left, min_right)
	}
}
