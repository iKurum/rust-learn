#![allow(dead_code)]

pub struct Solution;

impl Solution {
  /// 一个数对 (a,b) 的 **数对和** 等于 a + b 。**最大数对和** 是一个数对数组中最大的 **数对和** 。
  ///
  /// - 比方说，如果我们有数对 (1,5) ，(2,3) 和 (4,4)，**最大数对和** 为 max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8 。
  ///
  /// 给你一个长度为 **偶数** n 的数组 nums ，请你将 nums 中的元素分成 n / 2 个数对，使得：
  ///
  /// - nums 中每个元素 **恰好** 在 **一个** 数对中，且
  /// - **最大数对和** 的值 **最小** 。
  /// 请你在最优数对划分的方案下，返回最小的 **最大数对和** 。
  ///
  /// **提示：**
  ///```
  /// n == nums.length
  /// 2 <= n <= 10^5
  /// n 是 偶数 。
  /// 1 <= nums[i] <= 10^5
  /// ```
  pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut v = nums.clone();
    v.sort();
    let (mut ans, n) = (0, v.len());
    for i in 0..v.len() {
      ans = if ans >= v[i] + v[n - 1 - i] {
        ans
      } else {
        v[i] + v[n - 1 - i]
      };
    }

    ans as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn min_pair_sum() {
    assert_eq!(8, Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]));
  }
}
