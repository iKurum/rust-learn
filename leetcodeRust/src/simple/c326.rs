struct Solution;

impl Solution {
  /// 给定一个整数，写一个函数来判断它是否是 3 的幂次方。如果是，返回 true ；否则，返回 false 。
  ///
  /// 整数 n 是 3 的幂次方需满足：存在整数 x 使得 n == 3x
  ///
  /// 提示：
  /// ```
  ///   -2^31 <= n <= 2^31 - 1
  /// ```
  /// 进阶：
  /// ```
  ///   你能不使用循环或者递归来完成本题吗？
  /// ```
  fn is_power_of_three(n: i32) -> bool {
    // let mut n = n;
    // while n > 0 && n % 3 == 0 {
    //     n /= 3;
    // }

    // n == 1

    // 在题目给定的 32 位有符号整数的范围内，最大的 3 的幂为 3^19 =1162261467。我们只需要判断 n 是否是 3^19 的约数即可。
    n > 0 && 1162261467 % n == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn is_power_of_three() {
    assert_eq!(true, Solution::is_power_of_three(27));
  }
}
