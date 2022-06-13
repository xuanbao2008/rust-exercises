use std::fs;
use std::io;
fn main() {
  let contents = fs::read_to_string("word-list.txt").expect("Something went wrong");
  println!("Please input keyword:");
  let mut input_str = String::new();
  io::stdin().read_line(&mut input_str).unwrap();
  println!("Your searching keyword: {}", &input_str);
  let matched_word = contents.matches(&input_str.trim()).count();
  println!("The number of repeat: {} ", matched_word);
}
