use super::*;
use foo;

#[test]
fn unit_test_lib() {
    assert_eq!(main_gen(), testing::lib_gen());
}

#[test]
fn unit_test_foo() {
    assert_eq!(main_gen(), foo::foo_gen());
}
