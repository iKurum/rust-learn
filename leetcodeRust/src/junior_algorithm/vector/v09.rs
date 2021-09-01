#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 **和为目标值** target  的那 **两个** 整数，并返回它们的数组下标。
  ///
  /// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
  ///
  /// 你可以按任意顺序返回答案。
  ///
  /// 提示：
  /// ```
  ///   2 <= nums.length <= 10^4
  ///   -10^9 <= nums[i] <= 10^9
  ///   -10^9 <= target <= 10^9
  ///   只会存在一个有效答案
  /// ```
  fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
      let r = target - nums[i];
      println!("{:?}", map);
      if let Some(&l) = map.get(&nums[i]) {
        result.push(i as i32);
        result.push(l as i32);
        break;
      } else {
        map.insert(r, i);
      }
    }

    result
  }
}

pub fn test(nums: Vec<i32>, target: i32) -> Vec<i32> {
  Solution::two_sum(nums, target)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn two_sum() {
    assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
  }
}
