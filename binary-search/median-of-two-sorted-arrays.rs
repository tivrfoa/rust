// https://www.youtube.com/watch?v=LPFhl65R7ww

use std::cmp::max;
use std::cmp::min;

fn find_median_sorted_arrays(a: &[i32], b: &[i32]) -> f64 {

	if a.len() > b.len() {
		return find_median_sorted_arrays(&b, &a);
	}

	let x = a.len();
	let y = b.len();

	let mut low = 0;
	let mut high = x;
	while low <= high {
		let partition_x = (low + high) / 2;
		let partition_y = (x + y + 1) / 2 - partition_x;

		let max_left_x = if partition_x == 0 { i32::MIN } else { a[partition_x - 1] };
		let min_right_x = if partition_x == x { i32::MAX } else { a[partition_x] };

		let max_left_y = if partition_y == 0 { i32::MIN } else { b[partition_y - 1] };
		let min_right_y = if partition_y == y { i32::MAX } else { b[partition_y] };

		if max_left_x <= min_right_y && max_left_y <= min_right_x {
			if (x  + y) % 2 == 0 {
				return (max(max_left_x, max_left_y) + min(min_right_x, min_right_y)) as f64 / 2.0;
			} else {
				return  max(max_left_x, max_left_y) as f64;
			}
		} else if max_left_x > min_right_y {
			high = partition_x - 1;
		} else {
			low = partition_x + 1;
		}
	}

	panic!("Invalid arguments or implementation xD");
}

fn main() {

	let v1 = [1, 3, 8, 9, 15];
	let v2 = [7, 11, 19, 21, 18, 25];

	assert_eq!(find_median_sorted_arrays(&v1, &v2), 11.0);

	let v1 = [3, 5, 7, 9, 11, 16];
	let v2 = [23, 26, 31, 35];
	let d  = find_median_sorted_arrays(&v1, &v2);
	assert_eq!(d, 13.5);
}
