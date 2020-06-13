struct KthLargest {
    v: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
		println!("============== k = {}, nums = {:?}", k, nums);
		let k = k as usize;

		if k > nums.len() {
			nums.push(i32::min_value());
		}
        
        nums.sort();

		Self {
			v: nums[nums.len()-k..nums.len()].to_vec(),
		}
    }
    
    fn add(&mut self, val: i32) -> i32 {
		println!("Adding {}", val);
		println!("Prev self.v = {:?}", self.v);

		if val > self.v[0] {
			let idx = self.v.binary_search(&val).unwrap_or_else(|x| x) - 1;

			for i in 1..=idx {
				self.v[i-1] = self.v[i];
			}
			self.v[idx] = val;
		}
		println!("After self.v = {:?}", self.v);

        self.v[0]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
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
