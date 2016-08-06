pub fn get_true() -> bool {
    true
}

pub fn get_false() -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::get_true;

    #[test]
    fn get_true_shold_return_true() {
        assert_eq!(true, get_true());
    }

/*
    #[test]
    #[should_fail]
    fn it_should_pass_even_if_panic() {
        assert!(false);
    }
    */
}