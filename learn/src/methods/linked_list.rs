#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List<T> {
  pub val: T,
  pub next: Option<Box<List<T>>>,
}

impl<T: std::fmt::Display> List<T> {
  pub fn new(val: T) -> Self {
    List { val, next: None }
  }

  /// 插入到链表的最末端的位置
  pub fn push(&mut self, val: T) {
    if let Some(ref mut node) = self.next {
      node.push(val);
    } else {
      self.next = Some(Box::new(List::new(val)));
    }
  }

  /// 插入到链表的头部，作为新的链表中第一个存有数据的结点（又称为”首元结点”）
  pub fn unshift(&mut self, val: T)
  where
    T: std::clone::Clone,
  {
    let mut node = List::new(val);
    node.next = Some(Box::new(self.clone()));
    *self = node;
  }

  pub fn len(&self) -> u32
  where
    T: std::clone::Clone,
  {
    let mut node = self.clone();
    match node.next.take() {
      Some(tail) => 1 + tail.len(),
      None => 1,
    }
  }

  pub fn stringify(&self) -> String
  where
    T: std::clone::Clone,
  {
    let mut node = self.clone();
    let v = &node.val;
    match node.next.take() {
      Some(ref tail) => format!("{}, {}", v, tail.stringify()),
      None => format!("{}", node.val),
    }
  }
}
