#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
  ///
  /// 请必须使用时间复杂度为 O(log n) 的算法。
  /// 
  /// **提示:**
  /// ```
  /// 1 <= nums.length <= 10^4
  /// -10^4 <= nums[i] <= 10^4
  /// nums 为无重复元素的升序排列数组
  /// -10^4 <= target <= 10^4
  /// ```
  fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = (nums.len() - 1) as i32;

    // if nums[0] > target {
    //     return 0;
    // } else if nums[(r - 1) as usize] < target {
    //     return r;
    // }

    while l <= r {
      let m = l + (r - l) / 2;
      // if nums[m as usize] == target {
      //     return m;
      // }

      if nums[m as usize] < target {
        l = m + 1;
      } else {
        r = m - 1;
      }
    }

    l
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_insert() {
    assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
  }
}
