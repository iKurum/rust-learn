mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  let mut arr = vec!['h','e','l','l','o'];
  junior_algorithm::string::s01::test(&mut arr);
  println!("{:?}", arr);
}
