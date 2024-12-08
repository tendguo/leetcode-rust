use::std::cmp::max;
pub struct Solution;


impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_jump = 0;
        for (i, v) in nums.iter().enumerate() {
            if i > max_jump {
                return false;
            }

            max_jump = max(max_jump, i + *v as usize);
        }
        true
    }
}