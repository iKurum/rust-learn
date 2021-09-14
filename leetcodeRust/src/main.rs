mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  println!(
    "{:#?}",
    medium::c524::test(
      "abpcplea".to_string(),
      vec!["ale".to_string(),"apple".to_string(),"monkey".to_string(),"plea".to_string()]
    )
  );
}
