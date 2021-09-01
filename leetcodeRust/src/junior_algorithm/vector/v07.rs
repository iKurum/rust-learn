#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个由 **整数** 组成的 **非空** 数组所表示的非负整数，在该数的基础上加一。
  ///
  /// 最高位数字存放在数组的首位， 数组中每个元素只存储**单个**数字。
  ///
  /// 你可以假设除了整数 0 之外，这个整数不会以零开头。
  ///
  /// 提示：
  /// ```
  ///   1 <= digits.length <= 100
  ///   0 <= digits[i] <= 9
  /// ```
  fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = digits;
    for i in (0..res.len()).rev() {
      if res[i] == 9 {
        res[i] = 0;
      } else {
        res[i] += 1;
        return res;
      }
    }

    let mut r = vec![0; res.len() + 1];
    r[0] = 1;
    r
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn plus_one() {
    assert_eq!(vec![9, 0, 0, 0], Solution::plus_one(vec![8, 9, 9, 9]));
  }
}
