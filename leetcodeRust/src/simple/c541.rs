#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个字符串 s 和一个整数 k，从字符串开头算起，每 2k 个字符反转前 k 个字符。
  ///
  /// - 如果剩余字符少于 k 个，则将剩余字符全部反转。
  ///- 如果剩余字符小于 2k 但大于或等于 k 个，则反转前 k 个字符，其余字符保持原样。
  ///
  /// 提示：
  /// ```
  ///   1 <= s.length <= 10^4
  ///   s 仅由小写英文组成
  ///   1 <= k <= 10^4
  /// ```
  pub fn reverse_str(s: String, k: i32) -> String {
    let mut t = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < s.len() {
      let sub = if i + (k as usize) < s.len() {
        k as usize
      } else {
        s.len() - i
      };

      let mut j = 0;
      while j < sub / 2 {
        t.swap(j + i, sub + i - 1 - j);
        j += 1;
      }

      i += 2 * k as usize;
    }
    t.iter().cloned().collect::<String>()
  }
}

pub fn test(s: String, k: i32) -> String {
  Solution::reverse_str(s, k)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reverse_str() {
    assert_eq!(
      String::from("bacdfeg"),
      Solution::reverse_str(String::from("abcdefg"), 2)
    );
  }
}
