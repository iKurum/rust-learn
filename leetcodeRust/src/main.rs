mod hard;
mod medium;
mod simple;

fn main() {
  println!("{}", medium::c930::test(vec![1, 0, 1, 0, 1], 2));

  // let mut l1 = Some(Box::new(medium::c2::ListNode::new(3)));
  // let mut t = medium::c2::ListNode::new(4);
  // t.next = l1;
  // l1 = Some(Box::new(t));

  // t = medium::c2::ListNode::new(2);
  // t.next = l1;
  // l1 = Some(Box::new(t));

  // let mut l2 = Some(Box::new(medium::c2::ListNode::new(4)));
  // let mut t = medium::c2::ListNode::new(6);
  // t.next = l2;
  // l2 = Some(Box::new(t));

  // t = medium::c2::ListNode::new(5);
  // t.next = l2;
  // l2 = Some(Box::new(t));

  // let t1 = l1.clone();
  // let t2 = l2.clone();
  // println!("{:?}\n{:?}", t1, t2);

  // let t = medium::c2::Solution::add_two_numbers(l1, l2);
  // println!("{:?}", t);
}
