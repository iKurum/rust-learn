#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。
  ///
  /// 提示：你可以假定该字符串只包含小写字母。
  fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let s = s.chars().collect::<Vec<char>>();

    for i in 0..s.len() {
      let count = map.entry(s[i]).or_insert(0);
      *count += 1;
    }

    for i in 0..s.len() {
      if let Some(n) = map.get_mut(&s[i]) {
        if *n == 1 {
          return i as i32;
        }
      }
    }
    -1
  }
}

pub fn test(s: String) -> i32 {
  Solution::first_uniq_char(s)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn first_uniq_char() {
    assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
  }
}
