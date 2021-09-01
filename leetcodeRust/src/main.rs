mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  let mut v = vec![
    vec![5, 1, 9, 11],
    vec![2, 4, 8, 10],
    vec![13, 3, 6, 7],
    vec![15, 14, 12, 16],
  ];
  junior_algorithm::vector::v11::test(&mut v);
  println!("{:?}", v);
}
