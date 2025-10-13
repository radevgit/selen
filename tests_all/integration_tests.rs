//! Consolidated integration test
use selen::prelude::*;

#[test]
fn test_basic_solving() {
    let mut model = Model::default();
    let x = model.int(1, 10);
    model.new(x.eq(5));
    let result = model.solve();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get_int(x), 5);
}
