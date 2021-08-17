#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你一个字符串 s 表示一个学生的出勤记录，其中的每个字符用来标记当天的出勤情况（缺勤、迟到、到场）。记录中只含下面三种字符：
  ///
  /// - 'A'：Absent，缺勤
  /// - 'L'：Late，迟到
  /// - 'P'：Present，到场
  ///
  /// 如果学生能够 **同时** 满足下面两个条件，则可以获得出勤奖励：
  ///
  /// - 按 **总出勤** 计，学生缺勤（'A'）**严格** 少于两天。
  ///
  /// - 学生 **不会** 存在 **连续** 3 天或 3 天以上的迟到（'L'）记录。
  ///
  /// 如果学生可以获得出勤奖励，返回 true ；否则，返回 false 。
  ///
  /// 提示：
  /// ```
  ///   1 <= s.length <= 1000
  ///   s[i] 为 'A'、'L' 或 'P'
  /// ```
  fn check_record(s: String) -> bool {
    let (mut absents, mut lates) = (0, 0);

    for c in s.chars() {
      if c == 'A' {
        absents += 1;
        if absents >= 2 {
          return false;
        }
      }

      if c == 'L' {
        lates += 1;
        if lates >= 3 {
          return false;
        }
      } else {
        lates = 0;
      }
    }

    true
  }
}

pub fn test(s: String) -> bool {
  Solution::check_record(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_record() {
    assert_eq!(false, Solution::check_record(String::from("PPALLL")));
  }
}
