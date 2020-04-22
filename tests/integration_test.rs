#[test]
fn integration_test() {
    assert_eq!(testing::lib_gen(), testing::foo::foo_gen());
}
