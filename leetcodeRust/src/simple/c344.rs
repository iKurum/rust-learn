#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 char[] 的形式给出。
  ///
  /// 不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外空间解决这一问题。
  ///
  /// 你可以假设数组中的所有字符都是 ASCII 码表中的可打印字符。
  fn reverse_string(s: &mut Vec<char>) -> Vec<char> {
    let mut l: i32 = 0;
    let mut r: i32 = (s.len() - 1) as i32;

    while l <= r {
      let t = s[l as usize];
      s[l as usize] = s[r as usize];
      s[r as usize] = t;

      l += 1;
      r -= 1;
    }

    s.to_vec()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reverse_string() {
    assert_eq!(
      vec!['o', 'l', 'l', 'e', 'h'],
      Solution::reverse_string(&mut vec!['h', 'e', 'l', 'l', 'o'])
    );
  }
}
