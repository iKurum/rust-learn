mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  let result = junior_algorithm::vector::nine::test(vec![2,7,11,15], 9);
  println!("{:?}", result,);
}
