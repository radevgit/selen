use selen::prelude::*;

#[test]
#[ignore = "broken: or_all constraint not working, needs investigation"]
fn test_simple_or() {
    let mut model = Model::default();
    let a = model.bool();
    let b = model.bool();
    let c = model.bool();
    
    // Create OR constraint: at least one of a, b, c must be true
    let constraints = vec![a.eq(1), b.eq(1), c.eq(1)];
    println!("Created {} constraints", constraints.len());
    
    if let Some(or_constraint) = or_all(constraints) {
        println!("or_all succeeded, posting constraint");
        model.new(or_constraint);
    } else {
        panic!("or_all returned None!");
    }
    
    // Force a and b to be false
    model.new(a.eq(0));
    model.new(b.eq(0));
    
    // Should find solution with c=1
    match model.solve() {
        Ok(sol) => {
            println!("Solution found!");
            println!("a={}, b={}, c={}", sol.get_int(a), sol.get_int(b), sol.get_int(c));
            assert_eq!(sol.get_int(a), 0);
            assert_eq!(sol.get_int(b), 0);
            assert_eq!(sol.get_int(c), 1);
        }
        Err(e) => {
            panic!("No solution found: {:?}", e);
        }
    }
}
