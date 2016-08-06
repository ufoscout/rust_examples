
#[cfg(test)]
mod tests {
    
    extern crate fundamentals;
    
    //use super::get_true;

    #[test]
    fn fundamentals_get_true_shold_return_true() {
        assert_eq!(true, fundamentals::function::functions::get_true());
    }

/*
    #[test]
    #[should_fail]
    fn it_should_pass_even_if_panic() {
        assert!(false);
    }
    */
}