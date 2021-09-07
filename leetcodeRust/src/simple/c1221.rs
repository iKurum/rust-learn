#![allow(dead_code)]

struct Solution;

impl Solution {
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
