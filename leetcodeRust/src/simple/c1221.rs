#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 在一个 **平衡字符串** 中，'L' 和 'R' 字符的数量是相同的。
  ///
  /// 给你一个平衡字符串 s，请你将它分割成尽可能多的平衡字符串。
  ///
  /// 注意：分割得到的每个字符串都必须是平衡字符串。
  ///
  /// 返回可以通过分割得到的平衡字符串的 **最大数量** 。
  ///
  /// 提示：
  /// ```
  ///   1 <= s.length <= 1000
  ///   s[i] = 'L' 或 'R'
  ///   s 是一个 平衡 字符串
  /// ```
  fn balanced_string_split(s: String) -> i32 {
    let (mut result, mut temp) = (0, 0);

    for i in s.chars() {
      temp = if i == 'L' { temp - 1 } else { temp + 1 };

      if temp == 0 {
        result += 1;
      }
    }

    result
  }
}

pub fn test(s: String) -> i32 {
  Solution::balanced_string_split(s)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn balanced_string_split() {
    assert_eq!(4, Solution::balanced_string_split("RLRRLLRLRL".to_string()));
  }
}
