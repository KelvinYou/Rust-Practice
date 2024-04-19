fn is_palindrome(s: &str) -> bool {
  let s = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
  s == s.chars().rev().collect::<String>()
}

fn main() {
  println!("{}", is_palindrome("A man a plan a canal Panama"));
  println!("{}", is_palindrome("hello"));
}
