#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定两个数组，编写一个函数来计算它们的交集。
  fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (mut a, mut b) = (nums1, nums2);
    a.sort();
    b.sort();
    let mut result: Vec<i32> = Vec::new();
    let (mut r1, mut r2) = (0 as usize, 0 as usize);
    while r1 < a.len() && r2 < b.len() {
      if a[r1] == b[r2] {
        result.push(a[r1]);

        r1 += 1;
        r2 += 1;
      } else {
        if a[r1] > b[r2] {
          r2 += 1;
        } else {
          r1 += 1;
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersect() {
    assert_eq!(
      vec![4, 9],
      Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
    );
  }
}
