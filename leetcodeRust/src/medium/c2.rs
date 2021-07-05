// #![allow(dead_code)]

// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     ListNode { next: None, val }
//   }
// }

// pub struct Solution;

// impl Solution {
//   pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
//   ) -> Option<Box<ListNode>> {
//     let (mut t1, mut t2) = (l1, l2);
//     let mut tail = None;
//     let mut carry = 0;

//     loop {
//       let (mut n1, mut n2) = (0, 0);

//       if let Some(l) = &t1 {
//         n1 = l.val;
//         t1 = l.next.clone();
//       }

//       if let Some(l) = &t2 {
//         n2 = l.val;
//         t2 = l.next.clone();
//       }

//       let sum = (n1 + n2 + carry) % 10;
//       carry = (n1 + n2 + carry) / 10 as i32;

//       println!("nn: {}, {}", n1, n2);
//       match tail {
//         Some::<Box::<ListNode>>(ref t) => {
//           let mut tt = t;
//           loop {
//           if tt.next == None {
//             &tt.next = Some(Box::new(ListNode::new(sum)));
//             break;
//           } else {
//             if let Some(ref v) = tt.next {
//               tt = v;
//             }
//           }
//         }
//       },
//         None => {
//           tail = Some(Box::new(ListNode::new(sum)));
//         }
//       }

//       if None == t1 && None == t2 {
//         break;
//       }
//     }

//     if carry > 0 {
//       let mut t = ListNode::new(1);
//       t.next = tail;
//       tail = Some(Box::new(t));
//     }

//     tail
//   }
// }
