pub fn anagrams_for<'a>(s: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    return vec![];
}

#[test]
fn should_return_the_anagrams() {
    let inputs = ["tan", "stand", "at"]; // imputs is an array

    //&inputs is a slice; A slice is a portion of an array whose size is not known at compile time
    anagrams_for("ant", &inputs);
}

#[test]
fn test_detect_simple_anagram() {
  let inputs = ["tan", "stand", "at"];
  let outputs: Vec<&str> = vec!["tan"];
  assert_eq!(anagrams_for("ant", &inputs), outputs);
}

#[test]
fn test_no_matches() {
  let inputs = ["hello", "world", "zombies", "pants"];
  let outputs: Vec<&str> = vec![];
  assert_eq!(anagrams_for("diaper", &inputs), outputs);
}