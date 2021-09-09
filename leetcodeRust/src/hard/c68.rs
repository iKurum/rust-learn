#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给定一个单词数组和一个长度 maxWidth，重新排版单词，使其成为每行恰好有 maxWidth 个字符，且左右两端对齐的文本。
  ///
  /// 你应该使用“贪心算法”来放置给定的单词；也就是说，尽可能多地往每行中放置单词。必要时可用空格 ' ' 填充，使得每行恰好有 maxWidth 个字符。
  ///
  /// 要求尽可能均匀分配单词间的空格数量。如果某一行单词间的空格不能均匀分配，则左侧放置的空格数要多于右侧的空格数。
  ///
  /// 文本的最后一行应为左对齐，且单词之间不插入额外的空格。
  ///
  /// 说明:
  /// ```
  ///   单词是指由非空格字符组成的字符序列。
  ///   每个单词的长度大于 0，小于等于 maxWidth。
  ///   输入单词数组 words 至少包含一个单词。
  /// ```
  fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    let (mut right, n) = (0, words.len());

    loop {
      let left = right;
      let mut sum_len = 0;
      while right < n && sum_len + words[right].len() + right - left <= max_width as usize {
        sum_len += words[right].len();
        right += 1;
      }

      // 最后一行
      if right == n {
        let mut s = String::from(&words[left]);
        for i in (left + 1)..n {
          s += &format!(" {}", words[i]);
        }
        let l = s.len() as i32;
        ans.push(format!("{}{}", s, blank(max_width - l)));
        break;
      }

      let num_words = (right - left) as i32;
      let num_spaces = max_width - sum_len as i32;

      // 当前行只有一个单词
      if num_words == 1 {
        ans.push(format!("{}{}", words[left], blank(num_spaces)));
        continue;
      }

      let avg_spaces = num_spaces / (num_words - 1);
      let extra_spaces = (num_spaces % (num_words - 1)) as usize;
      let mut s1 = String::from(&words[left]);
      for i in (left + 1)..(left + extra_spaces + 1) {
        s1 += &format!("{}{}", blank(avg_spaces + 1), words[i]);
      }
      let mut s2 = String::from(&words[left + extra_spaces + 1]);
      for i in (left + extra_spaces + 2)..right {
        s2 += &format!("{}{}", blank(avg_spaces), words[i]);
      }
      ans.push(format!("{}{}{}", s1, blank(avg_spaces), s2));
    }

    ans
  }
}

fn blank(spaces: i32) -> String {
  let mut s = String::new();
  for _ in 0..spaces {
    s += " ";
  }
  s
}

pub fn test(words: Vec<String>, max_width: i32) -> Vec<String> {
  Solution::full_justify(words, max_width)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn full_justify() {
    assert_eq!(
      vec![
        "This    is    an".to_string(),
        "example  of text".to_string(),
        "justification.  ".to_string()
      ],
      Solution::full_justify(
        vec![
          "This".to_string(),
          "is".to_string(),
          "an".to_string(),
          "example".to_string(),
          "of".to_string(),
          "text".to_string(),
          "justification.".to_string()
        ],
        16
      )
    )
  }
}
