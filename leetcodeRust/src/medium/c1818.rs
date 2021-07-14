#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你两个正整数数组 nums1 和 nums2 ，数组的长度都是 n 。
  ///
  /// 数组 nums1 和 nums2 的 绝对差值和 定义为所有 |nums1[i] - nums2[i]|（0 <= i < n）的 总和（下标从 0 开始）。
  ///
  /// 你可以选用 nums1 中的 任意一个 元素来替换 nums1 中的 至多 一个元素，以 最小化 绝对差值和。
  ///
  /// 在替换数组 nums1 中最多一个元素 之后 ，返回最小绝对差值和。因为答案可能很大，所以需要对 10^9 + 7 取余 后返回。
  ///
  /// |x| 定义为：
  ///
  ///   - 如果 x >= 0 ，值为 x ，或者
  ///   - 如果 x <= 0 ，值为 -x
  /// 
  /// **提示：**
  ///
  /// ```
  /// n == nums1.length
  /// n == nums2.length
  /// 1 <= n <= 10^5
  /// 1 <= nums1[i], nums2[i] <= 10^5
  /// ```
  fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let m = 1000000007;
    let mut rec = nums1.clone();
    rec.sort();

    let (mut sum, mut maxn, n) = (0, 0, nums1.len());
    for i in 0..n {
      let diff = (nums1[i] - nums2[i]).abs();
      sum = (sum + diff) % m;

      let j = binary_search(&rec, nums2[i]);
      if j < n {
        maxn = if maxn - (diff - (rec[j] - nums2[i])) > 0 {
          maxn
        } else {
          diff - (rec[j] - nums2[i])
        };
      }

      if j > 0 {
        maxn = if maxn - (diff - (nums2[i] - rec[j - 1])) > 0 {
          maxn
        } else {
          diff - (nums2[i] - rec[j - 1])
        };
      }
    }
    (sum - maxn + m) % m
  }
}

fn binary_search(rec: &Vec<i32>, target: i32) -> usize {
  let mut low = 0;
  let mut high = rec.len() - 1;

  if rec[high] < target {
    return high + 1;
  }

  while low < high {
    let mid = (high - low) / 2 + low;
    if rec[mid] < target {
      low = mid + 1;
    } else {
      high = mid;
    }
  }

  low
}

pub fn test(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
  Solution::min_absolute_sum_diff(nums1, nums2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn min_absolute_sum_diff() {
    assert_eq!(
      9,
      Solution::min_absolute_sum_diff(vec![1, 28, 21], vec![9, 21, 20])
    );
  }
}
