// This is my solution to the Two Sum Leetcode question!
// It uses the two-pointer strategy(?) to find the two numbers that match up.

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len()-1 {
            for n in i+1..nums.len() {
            	if nums[i] + nums[n] == target {
            		return vec![i as i32, n as i32]
            	}
            }
        }
        panic!("No items sum to the target");
    }
}