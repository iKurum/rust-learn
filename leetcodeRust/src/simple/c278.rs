#![allow(dead_code)]

struct Solution {
  bad_version: i32,
}

// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
  fn new(bad_version: i32) -> Self {
    Self { bad_version }
  }

  #[allow(non_snake_case)]
  fn isBadVersion(&self, m: i32) -> bool {
    m >= self.bad_version
  }

  /// 你是产品经理，目前正在带领一个团队开发新的产品。不幸的是，你的产品的最新版本没有通过质量检测。由于每个版本都是基于之前的版本开发的，所以错误的版本之后的所有版本都是错的。
  ///
  /// 假设你有 n 个版本 [1, 2, ..., n]，你想找出导致之后所有版本出错的第一个错误的版本。
  ///
  /// 你可以通过调用 bool isBadVersion(version) 接口来判断版本号 version 是否在单元测试中出错。实现一个函数来查找第一个错误的版本。你应该尽量减少对调用 API 的次数。
  /// 
  /// **提示：**
  ///
  ///  - 1 <= bad <= n <= 2^31 - 1
  fn first_bad_version(&self, n: i32) -> i32 {
    let mut l: i32 = 1;
    let mut r: i32 = n;

    while l < r {
      let m = l + (r - l) / 2;
      if self.isBadVersion(m) {
        r = m;
      } else {
        l = m + 1;
      }
    }

    r
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn first_bad_version() {
    assert_eq!(4, Solution::new(4).first_bad_version(5))
  }
}
