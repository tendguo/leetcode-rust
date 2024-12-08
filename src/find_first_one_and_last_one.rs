/**
 *  输入：nums = [5,7,7,8,8,10], target = 8
    输出：[3,4]
    输入：nums = [5,7,7,8,8,10], target = 6
    输出：[-1,-1]
    输入：nums = [], target = 0
    输出：[-1,-1]
 */

pub fn solution() {
    let nums = vec![2, 2, 2, 3, 3, 7, 8, 9];
    let target = 7;

    println!("{:?}", Solution::search_range(nums, target))
}

// solution 1
// pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut left = -1;
//     let mut right = -1;
//     for (i, &v) in nums.iter().enumerate() {
//         if v == target {
//             if left == -1 {
//                 left = i as i32
//             }
//             right = i as i32;
//         }
//     }

//     vec![left, right]
// }

// solution 2
use std::cmp::Ordering;
struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![Solution::find_first_one(&nums, target), Solution::find_last_one(&nums, target)]
    }

    fn find_first_one(nums: &Vec<i32>, target: i32) -> i32 {
        if nums.len() <= 0 {
            return  -1;
        }
        let mut left = 0;
        let mut right = (nums.len() - 1) as i32;
    
        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid as i32 + 1,
                Ordering::Greater => right = mid as i32 - 1,
                Ordering::Equal => {
                    if mid == 0 || nums[mid - 1] != target {
                        return mid as i32;
                    }
    
                    right = mid as i32 - 1;
                }
            }
        }
        -1
    }
    
    fn find_last_one(nums: &Vec<i32>, target: i32) -> i32 {
        if nums.len() <= 0 {
            return  -1;
        }
        let mut left = 0;
        let mut right = (nums.len() - 1) as i32;
    
        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid as i32 + 1,
                Ordering::Greater => right = mid as i32 - 1,
                Ordering::Equal => {
                    if mid == nums.len() - 1 || nums[mid + 1] != target {
                        return mid as i32;
                    }
    
                    left = mid as i32 + 1;
                }
            }
        }
        -1
    }
    
}



