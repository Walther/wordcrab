extern crate wordcrab;
use wordcrab::*;

#[test]
fn empty_string() {
  let stats = analyse_string("");
  assert_eq!(stats.lines, 0);
  assert_eq!(stats.words, 0);
  assert_eq!(stats.chars, 0);
}

#[test]
fn empty_line() {
  let stats = analyse_string("\n");
  assert_eq!(stats.lines, 1);
  assert_eq!(stats.words, 0);
  assert_eq!(stats.chars, 1);
}

#[test]
fn short_ascii_line() {
  let stats = analyse_string("the quick brown fox jumps over the lazy dog");
  assert_eq!(stats.lines, 1);
  assert_eq!(stats.words, 9);
  assert_eq!(stats.chars, 43);
}

#[test]
fn short_ascii_paragraph() {
  let stats = analyse_string("lorem ipsum,\ndolor sit amet,\nconsectetur,\nadipiscing elit");
  assert_eq!(stats.lines, 4);
  assert_eq!(stats.words, 8);
  assert_eq!(stats.chars, 57);
}
