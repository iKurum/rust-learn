#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
  ///
  /// 说明:
  ///
  /// 1. 必须在原数组上操作，不能拷贝额外的数组。
  /// 2. 尽量减少操作次数。
  fn move_zeroes(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut l: usize = 0;
    let mut r: usize = 0;

    while r < nums.len() {
      if nums[r] != 0 {
        let temp = nums[l];
        nums[l] = nums[r];
        nums[r] = temp;
        l += 1;
      }
      r += 1;
    }

    nums.to_vec()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn move_zeroes() {
    assert_eq!(
      vec![1, 3, 12, 0, 0],
      Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12])
    );
  }
}
