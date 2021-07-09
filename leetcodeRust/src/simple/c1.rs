#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
  ///
  /// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
  ///
  /// 你可以按任意顺序返回答案。
  ///
  /// 提示：
  ///
  /// ```
  ///   2 <= nums.length <= 10^4
  ///   10^9 <= nums[i] <= 10^9
  ///   10^9 <= target <= 10^9
  ///   只会存在一个有效答案
  /// ```
  fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    // 双重循环
    // 'outer: for i in 0..nums.len() {
    //     for j in (i+1)..nums.len() {
    //         if nums[i] + nums[j] == target {
    //            v.push(i as i32);
    //            v.push(j as i32);
    //            if v.len() == 2 {
    //                break 'outer;
    //            }
    //         }
    //     }
    // }

    use std::collections::HashMap;
    let mut hash = HashMap::new();

    for i in 0..nums.len() {
      if hash.contains_key(&nums[i]) {
        if let Some(&n) = hash.get(&nums[i]) {
          v = vec![i as i32, n];
        }
      } else {
        hash.insert(target - nums[i], i as i32);
      }
    }

    v
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn two_sum() {
    let mut v = Solution::two_sum(vec![2, 7, 11, 15], 9);
    v.sort();
    assert_eq!(vec![0, 1], v);

    v = Solution::two_sum(vec![3,2,4], 6);
    v.sort();
    assert_eq!(vec![1, 2], v);

    v = Solution::two_sum(vec![3, 3], 6);
    v.sort();
    assert_eq!(vec![0, 1], v);
  }
}
