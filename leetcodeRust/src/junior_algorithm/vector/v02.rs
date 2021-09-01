#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个数组 prices ，其中 prices[i] 是一支给定股票第 i 天的价格。
  ///
  /// 设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。
  ///
  /// **注意：**你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
  fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
      return 0;
    }

    let (mut max, mut r) = (0, 1 as usize);
    while r < prices.len() {
      if prices[r] > prices[r - 1] {
        max += prices[r] - prices[r - 1];
      }
      r += 1;
    }

    max
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn max_profit() {
      assert_eq!(7, Solution::max_profit(vec![7,1,5,3,6,4]));
  }
}
