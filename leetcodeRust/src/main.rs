mod hard;
mod medium;
mod simple;

fn main() {
  println!("{:?}", medium::c189::test(&mut vec![1, 2, 3, 4, 5, 6, 7], 3));
}
