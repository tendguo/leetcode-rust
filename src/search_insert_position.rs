/**
 * 输入: nums = [1,3,5,6], target = 5
    输出: 2

    输入: nums = [1,3,5,6], target = 2
    输出: 1

    输入: nums = [1,3,5,6], target = 7
    输出: 4
 */

pub fn solution() {
    let nums = [2, 3, 3, 3, 3, 7, 8, 9];
    let target = 2;

    println!("{}", search_insert_position(nums, target))
}

// solution1
// fn search_insert_position<const T: usize>(nums: [i32; T], target: i32) -> usize {
//     for (i, v) in nums.iter().enumerate() {
//         if *v >= target {
//             return i;
//         }
//     }
//     nums.len()
// }

// solution2 二分查找
fn search_insert_position<const T: usize>(nums: [i32; T], target: i32) -> usize {
    let mut left: usize = 0;
    let mut right: usize = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > target {
            right = mid - 1;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            return mid;
        }
    }

    left

    // rust 自带的二分查找
    // nums.binary_search(&target).unwrap_or_else(|x| x) as i32
}
