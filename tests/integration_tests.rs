// extern crate we're testing, same as any other code would do.
extern crate quivi;

#[test]
fn test_add() {
    assert_eq!(quivi::add(3, 2), 6);
}
