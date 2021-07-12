#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 大餐 是指 恰好包含两道不同餐品 的一餐，其美味程度之和等于 2 的幂。
  ///
  /// 你可以搭配 任意 两道餐品做一顿大餐。
  ///
  /// 给你一个整数数组 deliciousness ，其中 deliciousness[i] 是第 i​​​​​​​​​​​​​​ 道餐品的美味程度，返回你可以用数组中的餐品做出的不同 大餐 的数量。结果需要对 **10^9 + 7** 取余。
  ///
  /// 注意，只要餐品下标不同，就可以认为是不同的餐品，即便它们的美味程度相同。
  /// 
  /// 提示：
  /// 
  ///```
  ///   1 <= deliciousness.length <= 10^5
  ///   0 <= deliciousness[i] <= 2^20
  ///```
  fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut ans: i64 = 0;
    let mut max_val = 0;
    for val in &deliciousness {
      max_val = if max_val > *val {
        max_val
      } else {
        *val
      };
    }

    let max_sum = max_val * 2;
    let mut scores: HashMap<i32, i32> = HashMap::new();
    for val in &deliciousness {
      let mut sum = 1;
      while sum <= max_sum {
        if let Some(t) = scores.get(&(sum - val)) {
          ans = ans.wrapping_add((*t).into());
        };

        sum = sum << 1;
      }

      let count = scores.entry(*val).or_insert(0);
      *count += 1;
    }

    (ans % 1000000007) as i32
  }
}

pub fn test(deliciousness: Vec<i32>) -> i32 {
  Solution::count_pairs(deliciousness)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_pairs() {
    assert_eq!(15, Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]));
  }
}
