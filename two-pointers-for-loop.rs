struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {

        // there's no need to consider any lower bar that comes after.

        let mut biggerL: i32 = 0;

        let len = height.len();
        let mut max: i32 = 0;

        for i in 0..len {
            if height[i] <= biggerL { continue; }
            biggerL = height[i];
            for j in (i..len).rev() {
                let lower = if height[i] < height[j] { height[i] } else { height[j] };
				let x = (j-i) as i32;
                let area = x * lower;
                if area > max { max = area; }
                if height[j] >= height[i] { break; }
            }
        }

        max
    }
}

fn main() {
	let area = Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
	println!("Area is {}", area);
}
