pub mod example;
pub mod function;

#[cfg(test)]
mod tests {
    use super::function;
    use super::function::functions;

    #[test]
    fn get_true_shold_return_true() {
        assert_eq!(true, functions::get_true());
    }

    #[test]
    fn get_false_shold_return_false() {
        assert!(!function::functions::get_false());
    }
}