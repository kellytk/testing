use super::*;

#[test]
fn unit_test_lib() {
    assert_eq!(main_gen(), lib::lib_gen());
}

#[test]
fn unit_test_foo() {
    assert_eq!(main_gen(), lib::foo::foo_gen());
}
