#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你一个二元数组 nums ，和一个整数 goal ，请你统计并返回有多少个和为 goal 的 非空 子数组。
  ///
  /// **子数组** 是数组的一段连续部分。
  /// 
  /// 提示：
  /// 
  /// ```
  ///   1 <= nums.length <= 3 * 104
  ///   nums[i] 不是 0 就是 1
  ///   0 <= goal <= nums.length
  /// ```
  pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let (mut l1, mut l2, mut r, mut s1, mut s2, mut ret): (i32, i32, i32, i32, i32, i32) = (0, 0, 0, 0, 0, 0);
    let s = nums.len() as i32;

    println!("s: {}", s);
    while r < s {
      println!("r: {}", r);
      s1 += nums[r as usize];
      println!("s1: {}", s1);
      while l1 <= r && s1 > goal {
        s1 -= nums[l1 as usize];
        l1 += 1;
      }

      s2 += nums[r as usize];
      while l2 <= r && s2 >= goal {
        s2 -= nums[l2 as usize];
        l2 += 1;
      }

      ret += l2 - l1;
      println!("ret: {}, {}, {}", ret, l1, l2);
      r += 1;
    }

    ret
  }
}

pub fn test(nums: Vec<i32>, goal: i32) -> i32 {
  Solution::num_subarrays_with_sum(nums, goal)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn num_subarrays_with_sum() {
    assert_eq!(4, Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
  }
}
