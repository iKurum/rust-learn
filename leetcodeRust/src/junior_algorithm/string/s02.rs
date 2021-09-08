#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
  ///
  /// 如果反转后整数超过 32 位的有符号整数的范围 [−2^31,  2^31 − 1] ，就返回 0。
  ///
  /// **假设环境不允许存储 64 位整数（有符号或无符号）**。
  /// 
  /// 提示：
  /// ```
  ///   -2^31 <= x <= 2^31 - 1
  /// ```
  fn reverse(x: i32) -> i32 {
    let (mut res, mut x) = (0, x);

    while x != 0 {
      let t = x % 10;
      let n = res * 10 + t;

      if (n - t) / 10 != res {
        return 0;
      }

      res = n;
      x /= 10;
    }

    res
  }
}

pub fn test(x: i32) -> i32 {
  Solution::reverse(x)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn reverse() {
      assert_eq!(-321, Solution::reverse(-123));
  }
}
