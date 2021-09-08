#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 char[] 的形式给出。
  ///
  /// 不要给另外的数组分配额外的空间，你必须**原地修改输入数组**、使用 O(1) 的额外空间解决这一问题。
  ///
  /// 你可以假设数组中的所有字符都是 ASCII 码表中的可打印字符。
  fn reverse_string(s: &mut Vec<char>) {
    let le = s.len();
    for i in 0..(le / 2) {
      let t = s[i];
      s[i] = s[le - 1 - i];
      s[le - 1 - i] = t;
    }
  }
}

pub fn test(s: &mut Vec<char>) {
  Solution::reverse_string(s)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn reverse_string() {
    let mut arr = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut arr);
    assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], arr);
  }
}
