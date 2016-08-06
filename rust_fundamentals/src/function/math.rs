pub fn add(first: i32, second: i32) -> i32 {
    first + second
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_add_integers() {
        assert_eq!(10, add(4,6));
    }

}
