// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  val: i32,
  next: Option<Box<ListNode>>,
}

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode { next: None, val }
//   }
// }

struct Solution;

impl Solution {
  /// 给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
  ///
  /// 提示：
  /// ```
  ///   链表中节点的数目范围是 [0, 5000]
  ///   -5000 <= Node.val <= 5000
  /// ```
  fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut lhs = head;
    let mut rhs = None;

    while let Some(mut node) = lhs {
      lhs = node.next.take();
      node.next = rhs;
      rhs = Some(node);
    }

    rhs
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn reverse_list() {
    assert_eq!(
      Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
          val: 4,
          next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
              val: 2,
              next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
          }))
        }))
      })),
      Solution::reverse_list(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
          val: 2,
          next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
              val: 4,
              next: Some(Box::new(ListNode { val: 5, next: None }))
            }))
          }))
        }))
      })))
    )
  }
}
