#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
  ///
  /// 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
  ///
  /// 提示:
  /// ```
  ///   1 <= s.length, t.length <= 5 * 10^4
  ///   s 和 t 仅包含小写字母
  /// ```
  fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    let mut a = vec![0; 26];
    let mut count = 0;

    for i in 0..s.len() {
      a[(s[i] as u32 - 'a' as u32) as usize] += 1;
      if a[(s[i] as u32 - 'a' as u32) as usize] == 1 {
        count += 1;
      }

      a[(t[i] as u32 - 'a' as u32) as usize] -= 1;
      if a[(t[i] as u32 - 'a' as u32) as usize] == 0 {
        count -= 1;
      }
    }

    return count == 0;
  }
}

pub fn test(s: String, t: String) -> bool {
  Solution::is_anagram(s, t)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn is_anagram() {
      assert_eq!(true, Solution::is_anagram("anagram".to_string(), "nagaram".to_string()))
  }
}
