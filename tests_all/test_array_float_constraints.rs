use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// Test: array_float_minimum
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_array_float_minimum_fixed_values() {
    let mut m = Model::default();
    
    let x = m.float(3.5, 3.5);
    let y = m.float(1.2, 1.2);
    let z = m.float(5.8, 5.8);
    
    let min_result = m.array_float_minimum(&[x, y, z])
        .expect("Should compute minimum");
    
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_result);
    
    assert!((min_val - 1.2).abs() < 1e-9, "Minimum should be 1.2");
}

#[test]
fn test_array_float_minimum_with_ranges() {
    let mut m = Model::default();
    
    let x = m.float(2.0, 5.0);
    let y = m.float(1.0, 3.0);
    let z = m.float(4.0, 6.0);
    
    let min_result = m.array_float_minimum(&[x, y, z])
        .expect("Should compute minimum");
    
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_result);
    
    // Minimum possible is 1.0 (from y), maximum possible is 3.0 (min of maxes)
    assert!(min_val >= 1.0 && min_val <= 3.0);
}

#[test]
fn test_array_float_minimum_negative() {
    let mut m = Model::default();
    
    let x = m.float(-2.5, -2.5);
    let y = m.float(1.0, 1.0);
    let z = m.float(-0.5, -0.5);
    
    let min_result = m.array_float_minimum(&[x, y, z])
        .expect("Should compute minimum");
    
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_result);
    
    assert!((min_val - (-2.5)).abs() < 1e-9, "Minimum should be -2.5");
}

#[test]
fn test_array_float_minimum_single_element() {
    let mut m = Model::default();
    
    let x = m.float(7.3, 7.3);
    
    let min_result = m.array_float_minimum(&[x])
        .expect("Should compute minimum");
    
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_result);
    
    assert!((min_val - 7.3).abs() < 1e-9, "Minimum of single element should be that element");
}

#[test]
fn test_array_float_minimum_empty_array() {
    let mut m = Model::default();
    
    let result = m.array_float_minimum(&[]);
    
    assert!(result.is_err(), "Should return error for empty array");
}

#[test]
fn test_array_float_minimum_with_constraint() {
    let mut m = Model::default();
    
    let x = m.float(5.0, 10.0);
    let y = m.float(6.0, 12.0);
    let z = m.float(7.0, 15.0);
    
    let min_result = m.array_float_minimum(&[x, y, z])
        .expect("Should compute minimum");
    
    // Verify the minimum is bounded correctly
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_result);
    
    // Minimum should be at least 5.0 (min of all mins) and at most 10.0 (min of all maxes)
    assert!(min_val >= 5.0, "Minimum should be at least 5.0");
    assert!(min_val <= 10.0, "Minimum should be at most 10.0");
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: array_float_maximum
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_array_float_maximum_fixed_values() {
    let mut m = Model::default();
    
    let x = m.float(3.5, 3.5);
    let y = m.float(1.2, 1.2);
    let z = m.float(5.8, 5.8);
    
    let max_result = m.array_float_maximum(&[x, y, z])
        .expect("Should compute maximum");
    
    let sol = m.solve().expect("Should have solution");
    let max_val = sol.get_float(max_result);
    
    assert!((max_val - 5.8).abs() < 1e-9, "Maximum should be 5.8");
}

#[test]
fn test_array_float_maximum_with_ranges() {
    let mut m = Model::default();
    
    let x = m.float(2.0, 5.0);
    let y = m.float(1.0, 3.0);
    let z = m.float(4.0, 6.0);
    
    let max_result = m.array_float_maximum(&[x, y, z])
        .expect("Should compute maximum");
    
    let sol = m.solve().expect("Should have solution");
    let max_val = sol.get_float(max_result);
    
    // Maximum possible is 6.0 (from z), minimum possible is 4.0 (max of mins)
    assert!(max_val >= 4.0 && max_val <= 6.0);
}

#[test]
fn test_array_float_maximum_negative() {
    let mut m = Model::default();
    
    let x = m.float(-2.5, -2.5);
    let y = m.float(-5.0, -5.0);
    let z = m.float(-0.5, -0.5);
    
    let max_result = m.array_float_maximum(&[x, y, z])
        .expect("Should compute maximum");
    
    let sol = m.solve().expect("Should have solution");
    let max_val = sol.get_float(max_result);
    
    assert!((max_val - (-0.5)).abs() < 1e-9, "Maximum should be -0.5");
}

#[test]
fn test_array_float_maximum_single_element() {
    let mut m = Model::default();
    
    let x = m.float(7.3, 7.3);
    
    let max_result = m.array_float_maximum(&[x])
        .expect("Should compute maximum");
    
    let sol = m.solve().expect("Should have solution");
    let max_val = sol.get_float(max_result);
    
    assert!((max_val - 7.3).abs() < 1e-9, "Maximum of single element should be that element");
}

#[test]
fn test_array_float_maximum_empty_array() {
    let mut m = Model::default();
    
    let result = m.array_float_maximum(&[]);
    
    assert!(result.is_err(), "Should return error for empty array");
}

#[test]
fn test_array_float_min_max_together() {
    let mut m = Model::default();
    
    let x = m.float(2.0, 8.0);
    let y = m.float(3.0, 7.0);
    let z = m.float(4.0, 6.0);
    
    let min_result = m.array_float_minimum(&[x, y, z])
        .expect("Should compute minimum");
    let max_result = m.array_float_maximum(&[x, y, z])
        .expect("Should compute maximum");
    
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_result);
    let max_val = sol.get_float(max_result);
    
    assert!(min_val <= max_val, "Minimum should be <= maximum");
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: array_float_element
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_array_float_element_fixed_index() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(10.5, 10.5),
        m.float(20.3, 20.3),
        m.float(30.7, 30.7),
    ];
    
    let index = m.int(1, 1); // Fixed at index 1
    let result = m.float(0.0, 50.0);
    
    m.array_float_element(index, &array, result);
    
    let sol = m.solve().expect("Should have solution");
    let result_val = sol.get_float(result);
    
    assert!((result_val - 20.3).abs() < 1e-9, "Should select array[1] = 20.3");
}

#[test]
fn test_array_float_element_variable_index() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(10.5, 10.5),
        m.float(20.3, 20.3),
        m.float(30.7, 30.7),
    ];
    
    let index = m.int(0, 2); // Can be any valid index
    let result = m.float(0.0, 50.0);
    
    m.array_float_element(index, &array, result);
    
    // Constrain result to be 30.7
    m.props.equals(result, Val::ValF(30.7));
    
    let sol = m.solve().expect("Should have solution");
    let index_val = sol.get_int(index);
    let result_val = sol.get_float(result);
    
    assert_eq!(index_val, 2, "Index should be 2");
    assert!((result_val - 30.7).abs() < 1e-9, "Result should be 30.7");
}

#[test]
fn test_array_float_element_bidirectional_propagation() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(5.0, 10.0),
        m.float(15.0, 20.0),
        m.float(25.0, 30.0),
    ];
    
    let index = m.int(0, 2);
    let result = m.float(0.0, 50.0);
    
    m.array_float_element(index, &array, result);
    
    let sol = m.solve().expect("Should have solution");
    let index_val = sol.get_int(index);
    let result_val = sol.get_float(result);
    
    // Verify the constraint is satisfied: result should match the selected array element
    match index_val {
        0 => assert!(result_val >= 5.0 && result_val <= 10.0),
        1 => assert!(result_val >= 15.0 && result_val <= 20.0),
        2 => assert!(result_val >= 25.0 && result_val <= 30.0),
        _ => panic!("Invalid index: {}", index_val),
    }
}

#[test]
fn test_array_float_element_zero_index() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(100.5, 100.5),
        m.float(200.3, 200.3),
    ];
    
    let index = m.int(0, 0); // Zero-based indexing
    let result = m.float(0.0, 500.0);
    
    m.array_float_element(index, &array, result);
    
    let sol = m.solve().expect("Should have solution");
    let result_val = sol.get_float(result);
    
    assert!((result_val - 100.5).abs() < 1e-9, "Should select array[0] = 100.5");
}

#[test]
fn test_array_float_element_negative_values() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(-10.5, -10.5),
        m.float(-5.3, -5.3),
        m.float(-2.7, -2.7),
    ];
    
    let index = m.int(0, 2);
    let result = m.float(-20.0, 0.0);
    
    m.array_float_element(index, &array, result);
    
    // Find the most negative value
    m.props.less_than_or_equals(result, Val::ValF(-10.0));
    
    let sol = m.solve().expect("Should have solution");
    let index_val = sol.get_int(index);
    let result_val = sol.get_float(result);
    
    assert_eq!(index_val, 0);
    assert!((result_val - (-10.5)).abs() < 1e-9);
}

#[test]
fn test_array_float_element_with_ranges() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(1.0, 5.0),
        m.float(10.0, 15.0),
        m.float(20.0, 25.0),
    ];
    
    let index = m.int(0, 2);
    let result = m.float(0.0, 30.0);
    
    m.array_float_element(index, &array, result);
    
    let sol = m.solve().expect("Should have solution");
    let index_val = sol.get_int(index);
    let result_val = sol.get_float(result);
    
    // Verify constraint is satisfied
    match index_val {
        0 => assert!(result_val >= 1.0 && result_val <= 5.0),
        1 => assert!(result_val >= 10.0 && result_val <= 15.0),
        2 => assert!(result_val >= 20.0 && result_val <= 25.0),
        _ => panic!("Invalid index"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Test: Combined Array Float Constraints
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_combined_min_max_element() {
    let mut m = Model::default();
    
    let array = vec![
        m.float(3.5, 3.5),
        m.float(1.2, 1.2),
        m.float(5.8, 5.8),
        m.float(2.4, 2.4),
    ];
    
    let _min_result = m.array_float_minimum(&array)
        .expect("Should compute minimum");
    let max_result = m.array_float_maximum(&array)
        .expect("Should compute maximum");
    
    let index = m.int(0, 3);
    let element_result = m.float(0.0, 10.0);
    
    m.array_float_element(index, &array, element_result);
    
    // Constrain: element should equal the maximum
    m.props.equals(element_result, max_result);
    
    let sol = m.solve().expect("Should have solution");
    let index_val = sol.get_int(index);
    let element_val = sol.get_float(element_result);
    let max_val = sol.get_float(max_result);
    
    assert_eq!(index_val, 2, "Should select index 2 (max value)");
    assert!((element_val - 5.8).abs() < 1e-9);
    assert!((max_val - 5.8).abs() < 1e-9);
}

#[test]
fn test_price_selection_scenario() {
    // Real-world scenario: Select a price from array based on conditions
    let mut m = Model::default();
    
    // Prices for different items
    let prices = vec![
        m.float(10.99, 10.99),  // Item 0
        m.float(15.49, 15.49),  // Item 1
        m.float(12.75, 12.75),  // Item 2
        m.float(8.50, 8.50),    // Item 3
    ];
    
    let selected_index = m.int(0, 3);
    let selected_price = m.float(0.0, 20.0);
    
    m.array_float_element(selected_index, &prices, selected_price);
    
    let sol = m.solve().expect("Should have solution");
    let index_val = sol.get_int(selected_index);
    let price_val = sol.get_float(selected_price);
    
    // Verify the selected price matches the array
    let expected_prices = [10.99, 15.49, 12.75, 8.50];
    assert!((price_val - expected_prices[index_val as usize]).abs() < 1e-9, 
            "Selected price should match array[{}] = {}", index_val, expected_prices[index_val as usize]);
}

#[test]
fn test_temperature_monitoring() {
    // Scenario: Find min/max temperature and which sensor read them
    let mut m = Model::default();
    
    let temps = vec![
        m.float(18.5, 22.3),  // Sensor 0
        m.float(19.2, 21.8),  // Sensor 1
        m.float(17.9, 23.1),  // Sensor 2
    ];
    
    let min_temp = m.array_float_minimum(&temps)
        .expect("Should find minimum");
    let max_temp = m.array_float_maximum(&temps)
        .expect("Should find maximum");
    
    let sol = m.solve().expect("Should have solution");
    let min_val = sol.get_float(min_temp);
    let max_val = sol.get_float(max_temp);
    
    // Min should be at least 17.9, max should be at most 23.1
    assert!(min_val >= 17.9 && min_val <= 21.8);
    assert!(max_val >= 19.2 && max_val <= 23.1);
    assert!(min_val <= max_val);
}
