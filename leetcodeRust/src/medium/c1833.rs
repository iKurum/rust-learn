#![allow(dead_code)]

pub struct Solution;

impl Solution {
  /// 夏日炎炎，小男孩 Tony 想买一些雪糕消消暑。
  /// 
  /// 商店中新到 n 支雪糕，用长度为 n 的数组 costs 表示雪糕的定价，其中 costs[i] 表示第 i 支雪糕的现金价格。Tony 一共有 coins 现金可以用于消费，他想要买尽可能多的雪糕。
  /// 
  /// 给你价格数组 costs 和现金量 coins ，请你计算并返回 Tony 用 coins 现金能够买到的雪糕的 最大数量。
  /// 
  /// 注意：Tony 可以按任意顺序购买雪糕。
  /// 
  /// 提示：
  /// 
  /// ```
  ///   costs.length == n
  ///   1 <= n <= 10^5
  ///   1 <= costs[i] <= 10^5
  ///   1 <= coins <= 10^8
  /// ```
  pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    // 贪心
    // let mut d = costs;
    // let mut cs = coins;
    // let mut i: i32 = 0;
    // d.sort();

    // for c in d {
    //   if cs >= c {
    //     i += 1;
    //     cs -= c;
    //   } else {
    //     break;
    //   }
    // }

    // i

    // 计数排序 + 贪心
    let mut freq = [0].repeat(100001);
    let mut cs = coins;
    for cost in costs {
      freq[cost as usize] += 1;
    }

    let mut count = 0;
    for i in 1..100001 {
      if cs >= i {
        let cur_count = if freq[i as usize] > (cs / i) as i32 {
          (cs / i) as i32
        } else {
          freq[i as usize]
        };

        if i == 100000 {
          println!(
            "cur: {}, {}, {}, {}",
            i,
            freq[i as usize],
            (cs / i) as i32,
            cur_count
          );
        }
        count += cur_count;
        cs -= i * cur_count;
        if i == 100000 {
          println!("rus: {}, {}", count, cs);
        }
      } else {
        break;
      }
    }

    count
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn max_ice_cream() {
    assert_eq!(4, Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7));
    assert_eq!(0, Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5));
    assert_eq!(6, Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20));
  }
}
