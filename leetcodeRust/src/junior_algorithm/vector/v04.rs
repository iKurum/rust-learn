#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个整数数组，判断是否存在重复元素。
  ///
  /// 如果存在一值在数组中出现至少两次，函数返回 true 。
  /// 
  /// 如果数组中每个元素都不相同，则返回 false 。
  fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let set = nums.iter().collect::<HashSet<&i32>>();
    !(nums.len() == set.len())
  }
}

pub fn test(nums: Vec<i32>) -> bool {
  Solution::contains_duplicate(nums)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn contains_duplicate() {
      assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
  }
}
