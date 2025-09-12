// create simple test here in without main function. 
use cspsolver::prelude::*;

#[test]
fn test_model() {
    // Create a new model
    let mut m2 = Model::default();

    // Create a variable x in [1, 10]
    let v = m2.int(1, 10);

    // Add constraint: x > 2.5
    m2.gt(v, float(2.5));

    // Solve the problem minimizing x
    let solution = m2.minimize(v).unwrap();
    let Val::ValI(x) = solution[v] else {
        assert!(false, "Expected integer value");
        return;
    };
    assert_eq!(x, 3);
}

#[test]
fn test_new_var_with_values() {
    // Create a new model
    let mut model = Model::default();

    // Create variables with predefined values
    let var1 = model.new_var_with_values(vec![2, 4, 6, 8]); // Even numbers
    let var2 = model.new_var_with_values(vec![1, 3, 5, 7]); // Odd numbers

    // Add constraint: variables must be different
    model.ne(var1, var2);

    // Solve the problem
    let solution = model.solve().unwrap();
    
    let Val::ValI(val1) = solution[var1] else {
        assert!(false, "Expected integer value for var1");
        return;
    };
    let Val::ValI(val2) = solution[var2] else {
        assert!(false, "Expected integer value for var2");
        return;
    };

    // Verify the solution
    assert!(val1 % 2 == 0, "var1 should be even");
    assert!(val2 % 2 == 1, "var2 should be odd");
    assert_ne!(val1, val2, "Variables should be different");
}
