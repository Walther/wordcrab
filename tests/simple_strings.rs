extern crate wordcrab;
use wordcrab::*;

mod analysis_options;
use analysis_options::*;

#[test]
fn empty_string() {
  let stats = analyse_string("", ANALYSIS_OPTIONS_LWC);
  assert_eq!(stats.lines, Some(0));
  assert_eq!(stats.words, Some(0));
  assert_eq!(stats.chars, Some(0));
}

#[test]
fn empty_line() {
  let stats = analyse_string("\n", ANALYSIS_OPTIONS_LWC);
  assert_eq!(stats.lines, Some(1));
  assert_eq!(stats.words, Some(0));
  assert_eq!(stats.chars, Some(1));
}

#[test]
fn short_ascii_line() {
  let stats = analyse_string(
    "the quick brown fox jumps over the lazy dog",
    ANALYSIS_OPTIONS_LWC,
  );
  assert_eq!(stats.lines, Some(1));
  assert_eq!(stats.words, Some(9));
  assert_eq!(stats.chars, Some(43));
}

#[test]
fn short_ascii_paragraph() {
  let stats = analyse_string(
    "lorem ipsum,\ndolor sit amet,\nconsectetur,\nadipiscing elit",
    ANALYSIS_OPTIONS_LWC,
  );
  assert_eq!(stats.lines, Some(4));
  assert_eq!(stats.words, Some(8));
  assert_eq!(stats.chars, Some(57));
}
