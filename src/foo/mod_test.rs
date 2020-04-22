use super::*;

#[test]
fn unit_test_lib() {
    assert_eq!(foo_gen(), crate::lib_gen());
}
