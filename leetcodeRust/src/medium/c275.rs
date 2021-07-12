#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一位研究者论文被引用次数的数组（被引用次数是非负整数），数组已经按照 升序排列 。编写一个方法，计算出研究者的 h 指数。
  ///
  /// h 指数的定义: “h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）总共有 h 篇论文分别被引用了至少 h 次。（其余的 N - h 篇论文每篇被引用次数不多于 h 次。）"
  ///
  /// 说明:
  ///
  /// 如果 h 有多有种可能的值 ，h 指数是其中最大的那个。
  fn h_index(citations: Vec<i32>) -> i32 {
    let n = citations.len() as i32;
    let (mut l, mut r) = (0, n - 1);

    while l <= r {
      let m = l + (r - l) / 2;
      if citations[m as usize] >= (n - m) {
        r = m - 1;
      } else {
        l = m + 1;
      }
    }

    n - l
  }
}

pub fn test(citations: Vec<i32>) -> i32 {
  Solution::h_index(citations)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn h_index() {
    assert_eq!(3, Solution::h_index(vec![0, 1, 3, 5, 6]));
  }
}
