use selen::prelude::*;

#[test]
fn element_constraint_with_computed_index_equality() {
    let mut m = Model::default();
    let arr = m.ints(3, 1, 10);
    let idx_aux = m.int(1, 3);
    let index = m.sub(idx_aux, int(1)); // index = idx_aux - 1
    let value = m.int(1, 10);
    m.element(&arr, index, value);
    m.new(idx_aux.eq(int(2))); // idx_aux = 2, so index should be 1
    m.new(value.eq(int(7)));   // value = 7
    let sol = m.solve().expect("Should find a solution");
    let idx = sol.get_int(index);
    let val = sol.get_int(value);
    let arr_val = sol.get_int(arr[idx as usize]);
    assert_eq!(idx, 1, "Index should be 1");
    assert_eq!(val, 7, "Value should be 7");
    assert_eq!(arr_val, 7, "arr[1] should be 7");
}

#[test]
fn element_constraint_with_prefixed_index() {
    let mut m = Model::default();
    let arr = m.ints(3, 1, 10);
    let idx_aux = m.int(2, 2); // Fixed to 2
    let index = m.sub(idx_aux, int(1)); // index = 1
    let value = m.int(7, 7); // Fixed to 7
    m.element(&arr, index, value);
    let sol = m.solve().expect("Should find a solution");
    let idx = sol.get_int(index);
    let val = sol.get_int(value);
    let arr_val = sol.get_int(arr[idx as usize]);
    assert_eq!(idx, 1, "Index should be 1");
    assert_eq!(val, 7, "Value should be 7");
    assert_eq!(arr_val, 7, "arr[1] should be 7");
}

#[test]
fn element_constraint_with_direct_index() {
    let mut m = Model::default();
    let arr = m.ints(3, 1, 10);
    let index = m.int(0, 2);
    let value = m.int(1, 10);
    m.element(&arr, index, value);
    m.new(index.eq(int(1)));
    m.new(value.eq(int(7)));
    let sol = m.solve().expect("Should find a solution");
    let idx = sol.get_int(index);
    let val = sol.get_int(value);
    let arr_val = sol.get_int(arr[idx as usize]);
    assert_eq!(idx, 1, "Index should be 1");
    assert_eq!(val, 7, "Value should be 7");
    assert_eq!(arr_val, 7, "arr[1] should be 7");
}
