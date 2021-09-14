#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你一个字符串 s 和一个字符串数组 dictionary 作为字典，找出并返回字典中最长的字符串，该字符串可以通过删除 s 中的某些字符得到。
  ///
  /// 如果答案不止一个，返回长度最长且字典序最小的字符串。如果答案不存在，则返回空字符串。
  ///
  /// 提示：
  /// ```
  ///   1 <= s.length <= 1000
  ///   1 <= dictionary.length <= 1000
  ///   1 <= dictionary[i].length <= 1000
  ///   s 和 dictionary[i] 仅由小写英文字母组成
  /// ```
  fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
    let s = s.chars().collect::<Vec<_>>();
    let mut res: Vec<String> = Vec::new();

    for t in dictionary {
      let (mut i, mut j) = (0, 0);
      let c = t.chars().collect::<Vec<_>>();
      while i < c.len() && j < s.len() {
        if c[i] == s[j] {
          i += 1;
        }
        j += 1;
      }

      if i == c.len() {
        res.push(t);
      }
    }

    res.sort_by(|a, b| {
      if a.len() != b.len() {
        b.len().cmp(&a.len())
      } else {
        a.cmp(b)
      }
    });
    println!("{:?}", res);

    if res.len() == 0 {
      return "".to_string();
    } else {
      return res[0].clone();
    }
  }
}

pub fn test(s: String, dictionary: Vec<String>) -> String {
  Solution::find_longest_word(s, dictionary)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_longest_word() {
    assert_eq!(
      "apple".to_string(),
      Solution::find_longest_word(
        "abpcplea".to_string(),
        vec![
          "ale".to_string(),
          "apple".to_string(),
          "monkey".to_string(),
          "plea".to_string()
        ]
      )
    )
  }
}
