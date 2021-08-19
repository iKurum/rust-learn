#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 编写一个函数，以字符串作为输入，反转该字符串中的元音字母。
  ///
  /// 提示：
  ///
  /// - 元音字母包含 aeiou 及其大写 。
  fn reverse_vowels(s: String) -> String {
    use std::collections::HashMap;
    let k = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let map = k.iter().zip(k.iter()).collect::<HashMap<&char, &char>>();
    let mut v = s.chars().collect::<Vec<char>>();
    let (mut l, mut r) = (0 as i32, (v.len() - 1) as i32);
    let mut temp: char = '0';
    while l < r {
      println!("form: {}-{}, {}-{}", l, v[l as usize], r, v[r as usize]);

      if temp == '0' {
        if let Some(&vl) = map.get(&v[l as usize]) {
          temp = *vl;
        } else {
          l += 1;
        }
      } else {
        if let Some(&vr) = map.get(&v[r as usize]) {
          println!("temp: {}", temp);

          v[r as usize] = temp;
          v[l as usize] = *vr;
          temp = '0';

          l += 1;
          r -= 1;
        } else {
          r -= 1;
        }
      }
    }
    v.iter().collect::<String>()
  }
}

pub fn test(s: String) -> String {
  Solution::reverse_vowels(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reverse_vowels() {
    assert_eq!(
      String::from("leotcede"),
      Solution::reverse_vowels(String::from("leetcode"))
    )
  }
}
