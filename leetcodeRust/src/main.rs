mod hard;
mod junior_algorithm;
mod medium;
mod simple;

fn main() {
  println!(
    "{:#?}",
    junior_algorithm::string::s04::test("anagram".to_string(), "aagaram".to_string())
  );
}
