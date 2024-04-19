fn count_vowels(input: &str) -> usize {
  let vowels = "aeiouAEIOU";
  input.chars().filter(|c| vowels.contains(*c)).count()
}

fn main() {
  println!("{}", count_vowels("Hello World")); // Should print: 3
  println!("{}", count_vowels("Rust is Fun")); // Should print: 2
}
