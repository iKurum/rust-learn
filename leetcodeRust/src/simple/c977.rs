#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
  ///
  /// 提示：
  ///```
  /// 1 <= nums.length <= 10^4
  /// -10^4 <= nums[i] <= 10^4
  /// nums 已按 非递减顺序 排序
  ///```
  /// 进阶：
  ///
  /// 请你设计时间复杂度为 O(n) 的算法解决本问题 - (提示：双指针)
  fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut negative: i32 = -1;
    for i in 0..n {
      if nums[i] < 0 {
        negative = i as i32;
      } else {
        break;
      }
    }

    let mut ans: Vec<i32> = Vec::new();
    let (mut i, mut j) = (negative, (negative + 1) as usize);
    while i >= 0 || j < n {
      if j == n {
        ans.push(nums[i as usize] * nums[i as usize]);
        i -= 1;
      } else if i < 0 {
        ans.push(nums[j] * nums[j]);
        j += 1;
      } else if nums[i as usize] * nums[i as usize] < nums[j] * nums[j] {
        ans.push(nums[i as usize] * nums[i as usize]);
        i -= 1;
      } else {
        ans.push(nums[j] * nums[j]);
        j += 1;
      }
    }

    ans
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sorted_squares() {
    assert_eq!(
      vec![0, 1, 9, 16, 100],
      Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
    );
  }
}
