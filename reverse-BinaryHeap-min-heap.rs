use std::{cmp::Reverse, collections::BinaryHeap};
struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    k: usize,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut pq = BinaryHeap::new();
        for x in nums {
            pq.push(Reverse(x));
            if pq.len() > k {
                pq.pop();
            }
        }
        Self { pq, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.k {
            self.pq.pop();
        }
        let x = *self.pq.peek().unwrap();
        x.0
    }
}

fn main() {

	test1();
	test2();
	test3();
	test4();
}

fn test1() {
	let k = 3;
	let nums = vec![4,5,8,2];
	let mut obj = KthLargest::new(k, nums);

	println!("{}", obj.add(3));   // returns 4
	println!("{}", obj.add(5));   // returns 5
	println!("{}", obj.add(10));  // returns 5
	println!("{}", obj.add(9));   // returns 8
	println!("{}", obj.add(4));   // returns 8
}

fn test2() {
	let k = 1;
	let nums = vec![];
	let mut obj = KthLargest::new(k, nums);

	assert_eq!(obj.add(-3), -3);
	assert_eq!(obj.add(-2), -2);
	assert_eq!(obj.add(-4), -2);
	assert_eq!(obj.add(0), 0);
	assert_eq!(obj.add(4), 4);
}

fn test3() {
	let k = 2;
	let nums = vec![0];
	let mut obj = KthLargest::new(k, nums);

	assert_eq!(obj.add(-1), -1);
	assert_eq!(obj.add(1), 0);
	assert_eq!(obj.add(-2), 0);
	assert_eq!(obj.add(3), 1);
	assert_eq!(obj.add(4), 3);
}

fn test4() {
	let k = 3;
	let nums = vec![5, -1];
	let mut obj = KthLargest::new(k, nums);

	assert_eq!(obj.add(2), -1);
	assert_eq!(obj.add(1), 1);
	assert_eq!(obj.add(-1), 1);
	assert_eq!(obj.add(3), 2);
	assert_eq!(obj.add(4), 3);
}
