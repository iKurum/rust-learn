#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 峰值元素是指其值严格大于左右相邻值的元素。
  ///
  /// 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
  ///
  /// 你可以假设 nums[-1] = nums[n] = -∞ 。
  ///
  /// 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。
  ///
  /// 提示：
  /// ```
  ///   1 <= nums.length <= 1000
  ///   -2^31 <= nums[i] <= 2^31 - 1
  ///   对于所有有效的 i 都有 nums[i] != nums[i + 1]
  /// ```
  fn find_peak_element(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
      let m = l + ((r - l) >> 1);
      if nums[m] < nums[m + 1] {
        l = m + 1;
      } else {
        r = m;
      }
    }
    l as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_peak_element() {
    assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
  }
}
