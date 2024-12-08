use::std::cmp::max;
pub struct Solution;


impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut max_position = 0;
        let mut step = 0;

        for i in 0..nums.len() - 1 {
            max_position = max(max_position, i + nums[i] as usize);

            if i == end {
                end = max_position;
                step += 1;
            }
        }
        step
    }
}