#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。
  ///
  /// **提示：**
  ///
  /// - 你可以假设 nums 中的所有元素是不重复的。
  /// - n 将在 [1, 10000]之间。
  /// - nums 的每个元素都将在 [-9999, 9999]之间。
  fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = (nums.len() - 1) as i32;
    let mut result: i32 = -1;

    while l <= r {
      let m = l + (r - l) / 2;

      if nums[m as usize] == target {
        result = m;
        break;
      }

      if nums[m as usize] < target {
        l = (m + 1) as i32;
      } else {
        r = (m - 1) as i32;
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search() {
    assert_eq!(-1, Solution::search(vec![5], -5))
  }
}
