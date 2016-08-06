mod functions {

    pub fn getTrue() -> bool {
        true
    }

    pub fn getFalse() -> bool {
        return false;
    }
}


#[test]
fn prime_factors_of_two() {
    assert_eq!(prime_factors(2), [2]);
}