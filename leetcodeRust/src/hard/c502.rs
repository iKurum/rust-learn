#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 假设 力扣（LeetCode）即将开始 IPO 。为了以更高的价格将股票卖给风险投资公司，力扣 希望在 IPO 之前开展一些项目以增加其资本。 由于资源有限，它只能在 IPO 之前完成最多 k 个不同的项目。帮助 力扣 设计完成最多 k 个不同项目后得到最大总资本的方式。
  ///
  /// 给你 n 个项目。对于每个项目 i ，它都有一个纯利润 profits[i] ，和启动该项目需要的最小资本 capital[i] 。
  ///
  /// 最初，你的资本为 w 。当你完成一个项目时，你将获得纯利润，且利润将被添加到你的总资本中。
  ///
  /// 总而言之，从给定项目中选择 最多 k 个不同项目的列表，以 最大化最终资本 ，并输出最终可获得的最多资本。
  ///
  /// 答案保证在 32 位有符号整数范围内。
  /// 提示：
  /// ```
  ///   1 <= k <= 10^5
  ///   0 <= w <= 10^9
  ///   n == profits.length
  ///   n == capital.length
  ///   1 <= n <= 10^5
  ///   0 <= profits[i] <= 10^4
  ///   0 <= capital[i] <= 10^9
  /// ```
  fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    #[derive(Debug, Clone)]
    struct Pair {
      c: i32,
      p: i32,
    }
    let mut arr: Vec<Pair> = vec![Pair { c: 0, p: 0 }; profits.len()];

    for i in 0..profits.len() {
      arr[i] = Pair {
        c: capital[i],
        p: profits[i],
      };
    }

    arr.sort_by(|a, b| a.c.partial_cmp(&b.c).unwrap());

    let (mut curr, mut result) = (0, w);
    let mut pq: Vec<i32> = Vec::new();
    for _ in 0..k {
      let mut b = false;
      while curr < profits.len() && arr[curr].c <= result {
        pq.push(arr[curr].p);
        b = true;
        curr += 1;
      }

      if !pq.is_empty() {
        if b {
          pq.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        }

        if let Some(t) = pq.pop() {
          result += t;
        }
      } else {
        break;
      }
    }

    result
  }
}

pub fn test(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
  Solution::find_maximized_capital(k, w, profits, capital)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_maximized_capital() {
    assert_eq!(
      4,
      Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
    );
  }
}
