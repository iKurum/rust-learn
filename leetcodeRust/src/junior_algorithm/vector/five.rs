#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。
  ///
  /// 找出那个只出现了一次的元素。
  ///
  /// 题解思路（异或）
  ///
  /// 使用异或运算，将所有值进行异或
  ///
  /// - 异或运算，相异为真，相同为假，所以 a^a = 0 ;0^a = a
  /// - 因为异或运算 满足交换律 a^b^a = a^a^b = b 所以数组经过异或运算，单独的值就剩下了
  fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
      result ^= nums[i];
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_number() {
    assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
  }
}
