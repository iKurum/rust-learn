#![allow(dead_code)]

struct Solution;

impl Solution {
    /// 统计一个数字在排序数组中出现的次数。
    /// 
    /// **限制：**
    ///
    /// 0 <= 数组长度 <= 50000
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // let mut has_a = false;
        // let (mut a, mut b) = (0, 0);
        // for i in 0..nums.len() {
        //     if (nums[i] == target) && !has_a {
        //         a = i;
        //         has_a = true;
        //     }
        //     if nums[i] > target {
        //         b = i;
        //         break;
        //     }

        //     if (i == nums.len() - 1) && nums[i] == target {
        //         b = nums.len();
        //     }
        // }

        // if !has_a {
        //     0
        // } else {
        //     (b - a) as i32
        // }

        // 二分
        let mut ans = 0;
        if nums.len() == 0 {
            return 0;
        }

        let left_idx = binary_search(&nums, target, true) as usize;
        let right_idx = (binary_search(&nums, target, false) - 1) as usize;

        if left_idx <= right_idx
            && right_idx < nums.len()
            && nums[left_idx] == target
            && nums[right_idx] == target
        {
            ans = right_idx - left_idx + 1;
        }

        ans as i32
    }
}

fn binary_search(nums: &Vec<i32>, target: i32, lower: bool) -> i32 {
    let (mut left, mut right, mut ans) = (0, nums.len() - 1, nums.len());
    while left <= right {
        let mid = (left + right) / 2;
        println!("{}, {}, {}, {}", lower, left, right, mid);

        if nums[mid] > target || (lower && nums[mid] >= target) {
            ans = mid;

            if mid >= 1 {
                right = mid - 1;
            } else {
                break;
            };
        } else {
            left = mid + 1;
        }
    }

    ans as i32
}

pub fn test(nums: Vec<i32>, target: i32) -> i32 {
    Solution::search(nums, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        assert_eq!(1, Solution::search(vec![1, 4], 4));
    }
}
