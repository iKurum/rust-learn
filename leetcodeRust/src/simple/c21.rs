// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  val: i32,
  next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution;
impl Solution {
  /// 将两个升序链表合并为一个新的 **升序** 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
  ///
  /// 提示：
  /// ```
  ///   两个链表的节点数目范围是 [0, 50]
  ///   -100 <= Node.val <= 100
  ///   l1 和 l2 均按 非递减顺序 排列
  /// ```
  fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let (mut lhs, mut rhs) = (l1, l2);
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    while let (Some(lnode), Some(rnode)) = (lhs.as_ref(), rhs.as_ref()) {
      if lnode.val <= rnode.val {
        tail.next = lhs;
        tail = tail.next.as_mut().unwrap();
        lhs = tail.next.take();
      } else {
        tail.next = rhs;
        tail = tail.next.as_mut().unwrap();
        rhs = tail.next.take();
      }
    }

    tail.next = if lhs.is_some() { lhs } else { rhs };
    head.next
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn merge_two_lists() {
    assert_eq!(
      Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
          val: 1,
          next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
              val: 3,
              next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 4, next: None }))
              }))
            }))
          }))
        }))
      })),
      Solution::merge_two_lists(
        Some(Box::new(ListNode {
          val: 1,
          next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None }))
          }))
        })),
        Some(Box::new(ListNode {
          val: 1,
          next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None }))
          }))
        }))
      )
    );
  }
}
