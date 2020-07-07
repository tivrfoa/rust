use std::thread;

static MAX_THREADS: usize = 4;

fn main() {
	
	let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

	let mut children = vec![];
	
	// let chuncked_data = data.split_whitespace();
	
	let chunck_per_thread = data.len() / MAX_THREADS;
	
	for t in 0..MAX_THREADS {
		let l = t * chunck_per_thread;
		let r = l + chunck_per_thread;
		// println!("l = {} r = {}", l, r);
		let data_segment = if t + 1 == MAX_THREADS {
			&data[l..]
		} else {
			&data[l..r]
		};
		
		println!("data segment {} is \"{}\"", t, data_segment);
		
		children.push(thread::spawn(move || -> u32 {
			let result = data_segment
					.chars()
					.map(|c| match c.to_digit(10) {
						Some(n) => n,
						None => 0,
					})
					.sum();
			
			println!("processed segment {}, result = {}", t, result);
			
			result
		}));
	}
	
	let mut intermediate_sums: Vec<u32> = vec![];
	for child in children {
		intermediate_sums.push(child.join().unwrap());
	}
	
	let final_result: u32 = intermediate_sums.iter().sum();
	
	assert_eq!(final_result, 1342);
	
	println!("Final sum result: {}", final_result);
}
