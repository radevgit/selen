use selen::prelude::*;

#[test]
fn count_var_basic() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let target = m.int(2, 4);  // Target is variable, can be 2, 3, or 4
    let count = m.int(0, 3);
    
    m.count_var(&vars, target, count);
    m.new(target.eq(int(3)));  // Fix target to 3
    m.new(vars[0].eq(int(3))); // v1 = 3
    m.new(vars[1].eq(int(3))); // v2 = 3
    m.new(vars[2].ne(int(3))); // v3 != 3
    
    let sol = m.solve().expect("Should find a solution");
    assert_eq!(sol.get_int(count), 2, "Count should be 2 (two variables equal target)");
    assert_eq!(sol.get_int(target), 3, "Target should be 3");
}

#[test]
fn count_var_with_computed_target() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let x = m.int(1, 3);
    let target = m.add(x, int(1));  // target = x + 1
    let count = m.int(0, 3);
    
    m.count_var(&vars, target, count);
    m.new(x.eq(int(2)));           // x = 2, so target = 3
    m.new(vars[0].eq(int(3)));    // v1 = 3
    m.new(vars[1].eq(int(3)));    // v2 = 3
    
    let sol = m.solve().expect("Should find a solution");
    assert_eq!(sol.get_int(target), 3, "Target should be 3");
    assert_eq!(sol.get_int(count), 2, "Count should be 2");
}

#[test]
fn count_var_no_matches() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3)];
    let target = m.int(4, 5);  // Target in range 4-5
    let count = m.int(0, 3);
    
    m.count_var(&vars, target, count);
    m.new(target.eq(int(5)));  // target = 5
    
    let sol = m.solve().expect("Should find a solution");
    assert_eq!(sol.get_int(count), 0, "Count should be 0 (no matches)");
}

#[test]
fn count_var_all_match() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    let target = m.int(1, 5);
    let count = m.int(3, 3);  // Constraint: exactly 3 must match
    
    m.count_var(&vars, target, count);
    m.new(target.eq(int(2)));  // target = 2
    
    let sol = m.solve().expect("Should find a solution");
    assert_eq!(sol.get_int(count), 3, "Count should be 3");
    // All variables should equal the target
    assert_eq!(sol.get_int(vars[0]), 2);
    assert_eq!(sol.get_int(vars[1]), 2);
    assert_eq!(sol.get_int(vars[2]), 2);
}

#[test]
fn count_var_dynamic_domain_overlap() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 10), m.int(1, 10), m.int(1, 10), m.int(1, 10)];
    let target = m.int(5, 7);  // Target can be 5, 6, or 7
    let count = m.int(2, 3);   // Count should be 2 or 3
    
    m.count_var(&vars, target, count);
    
    let sol = m.solve().expect("Should find a solution");
    // Count should be between 2 and 3
    let count_val = sol.get_int(count);
    assert!(count_val == 2 || count_val == 3, "Count should be 2 or 3, got {}", count_val);
}
