fn reverse_words(input: &str) -> String {
  let mut words: Vec<&str> = input.split_whitespace().collect();

  // println!("words: {:?}", words);
  words.reverse();
  words.join(" ")
}

fn main() {
  println!("{}", reverse_words("hello world")); // Should print: "world hello"
  println!("{}", reverse_words("rust is fun")); // Should print: "fun is rust"
}
