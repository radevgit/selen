use selen::core::ValueAccessError;
use selen::variables::Val;
use selen::api::Model;

#[test]
fn test_safe_solution_access() {
    // Create a model and add variables to get proper VarIds
    let mut model = Model::default();
    let int_var = model.new_var(Val::ValI(42), Val::ValI(42)); // Fixed domain of 42
    let float_var = model.new_var(Val::ValF(3.14), Val::ValF(3.14)); // Fixed domain of 3.14
    
    // Solve to get a solution
    let result = model.solve().expect("Should have a solution");
    let solution = result;
    
    // Test backward compatible (panicking) methods
    assert_eq!(solution.get_int(int_var), 42);
    assert_eq!(solution.get_float(float_var), 3.14);
    
    // Test safe access methods
    assert_eq!(solution.try_get_int(int_var), Ok(42));
    assert!(matches!(
        solution.try_get_int(float_var), 
        Err(ValueAccessError::ExpectedInteger { .. })
    ));
    
    assert_eq!(solution.try_get_float(float_var), Ok(3.14));
    assert!(matches!(
        solution.try_get_float(int_var), 
        Err(ValueAccessError::ExpectedFloat { .. })
    ));
    
    // Test try_get generic method
    let result: Result<i32, ValueAccessError> = solution.try_get(int_var);
    assert_eq!(result, Ok(42));
    
    let result: Result<f64, ValueAccessError> = solution.try_get(float_var);
    assert_eq!(result, Ok(3.14));
    
    // Test additional Option-based methods
    assert_eq!(solution.as_int(int_var), Some(42));
    assert_eq!(solution.as_int(float_var), None);
    assert_eq!(solution.as_float(float_var), Some(3.14));
    assert_eq!(solution.as_float(int_var), None);
}

#[test] 
#[should_panic]
fn test_unchecked_methods_still_panic() {
    // Create a model with a float variable
    let mut model = Model::default();
    let float_var = model.new_var(Val::ValF(3.14), Val::ValF(3.14)); // Fixed domain of 3.14
    
    // Solve to get a solution
    let result = model.solve().expect("Should have a solution");
    let solution = result;
    
    // This should panic because we're trying to get an int from a float variable
    _ = solution.get_int_unchecked(float_var);
}