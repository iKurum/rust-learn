#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个数组，将数组中的元素向右移动 k 个位置，其中 k 是非负数。
  ///
  /// 进阶：
  ///
  /// - 尽可能想出更多的解决方案，至少有三种不同的方法可以解决这个问题。
  /// - 你可以使用空间复杂度为 O(1) 的 **原地** 算法解决这个问题吗？
  ///
  /// 提示：
  ///```
  /// 1 <= nums.length <= 2 * 10^4
  /// -2^31 <= nums[i] <= 2^31 - 1
  /// 0 <= k <= 10^5
  ///```
  fn rotate(nums: &mut Vec<i32>, k: i32) -> Vec<i32> {
    let k = (k as usize) % nums.len();
    reverse(nums, 0, (nums.len() - 1) as i32);
    reverse(nums, 0, (k - 1) as i32);
    reverse(nums, k as i32, (nums.len() - 1) as i32);

    nums.to_vec()
  }
}

fn reverse(nums: &mut Vec<i32>, mut start: i32, mut end: i32) {
  while start < end {
    let temp = nums[start as usize];
    nums[start as usize] = nums[end as usize];
    nums[end as usize] = temp;
    start += 1;
    end -= 1;
  }
}

pub fn test(nums: &mut Vec<i32>, k: i32) -> Vec<i32> {
  Solution::rotate(nums, k)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn rotate() {
    assert_eq!(
      vec![5, 6, 7, 1, 2, 3, 4],
      Solution::rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3)
    );
  }
}
