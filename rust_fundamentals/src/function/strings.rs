
/// String is a mutable string
pub fn method_with_string(source: String) {
}

/// &str is an immutable string
pub fn method_with_str(source: &str) {
}

#[test]
fn use_different_string_types() {
    let s = "String!"; //s: &'static str
    method_with_str(s); //works, since s is a &str
    method_with_string(s.to_string()); //works, since we just cast s to be a String
}