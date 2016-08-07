pub fn anagrams_for(s: &str, inputs: &[&str]) {
    // Amazing logic TBD
}

#[test]
fn should_return_the_anagrams() {
    let inputs = ["tan", "stand", "at"]; // imputs is an array

    //&inputs is a slice; A slice is a portion of an array whose size is not known at compile time
    anagrams_for("ant", &inputs);
}
