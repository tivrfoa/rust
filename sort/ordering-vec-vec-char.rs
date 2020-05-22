use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<Vec<char>> = nums
            .into_iter()
            .map(|n| n.to_string().chars().collect())
            .collect();
        
        nums.sort_by(|a, b| {
            for i in 0..a.len() + b.len() {
                let c1 = if i < a.len() {
                    a[i]
                } else {
                    b[i - a.len()]
                };
                let c2 = if i < b.len() {
                    b[i]
                } else {
                    a[i - b.len()]
                };
                if c1 != c2 {
                    return c2.partial_cmp(&c1).unwrap();
                }
            }
            Ordering::Equal
        });
        
        let nums: Vec<String> = nums.into_iter().map(|n| n.iter().collect()).collect();
        if nums.len() == 0 {
            "".to_string()
        } else if &nums[0] == "0" {
            "0".to_string()
        } else {
            nums.join("")
        }
    }
}
