#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
  ///
  /// 说明:
  /// 1. 必须在原数组上操作，不能拷贝额外的数组。
  /// 2. 尽量减少操作次数。
  fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0 as usize;
    for j in 0..nums.len() {
      if nums[j] != 0 {
        nums.swap(i, j);
        i += 1;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn move_zeroes() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(vec![1, 3, 12, 0, 0], nums);
  }
}
