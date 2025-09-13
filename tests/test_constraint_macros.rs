use cspsolver::prelude::*;

#[test]
fn test_sum_function_support() {
    let mut m = Model::new();
    
    // Test sum() with variable list
    let vars = vec![m.int(1, 10), m.int(1, 10), m.int(1, 10)];
    post!(m, sum(vars) == int(15));
    
    // Test sum() with direct expressions  
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    post!(m, sum([x, y, z]) == int(20));
    
    // Test with float variables
    let fx = m.float(1.0, 10.0);
    let fy = m.float(1.0, 10.0);
    post!(m, sum([fx, fy]) == float(15.5));
    
    // Test sum() as part of larger expression
    post!(m, sum([x, y]) + z == int(25));
    post!(m, sum([x, y]) * int(2) == int(30));
    
    // Test sum() in inequalities
    post!(m, sum([x, y, z]) <= int(20));
    post!(m, sum([x, y, z]) >= int(10));
    
    println!("Sum function support tests passed!");
}

#[test]
fn test_float_constants_math_functions() {
    let mut m = Model::new();
    
    let x = m.float(-10.0, 10.0);
    let y = m.float(-10.0, 10.0);
    let z = m.float(-10.0, 10.0);
    
    // Test abs() with float constants
    post!(m, abs(x) <= float(5.5));
    post!(m, abs(x + float(2.5)) == float(7.0));
    
    // Test min() with float constants  
    post!(m, min(x, float(3.5)) == y);
    post!(m, min(float(1.5), y) <= float(5.0));
    
    // Test max() with float constants
    post!(m, max(x, float(2.5)) == z);
    post!(m, max(float(4.5), y) >= float(3.0));
    
    // Test nested expressions
    post!(m, abs(min(x, float(3.0))) <= max(y, float(2.0)));
    
    // Test complex expressions
    post!(m, abs(x) + min(y, float(5.5)) == max(z, float(1.5)));
    
    println!("Float constants with math functions tests passed!");
}

#[test] 
fn test_boolean_logic_functions() {
    let mut m = Model::new();
    
    let a = m.int(0, 1); // Boolean variable
    let b = m.int(0, 1); // Boolean variable
    let c = m.int(0, 1); // Boolean variable
    
    // Test traditional logical operators (clean syntax)
    post!(m, and(a, b) == c);
    post!(m, or(a, b) == c);
    post!(m, not(a) == b);
    
    // Test complex boolean expressions using traditional style
    post!(m, and(a, or(b, c)) == int(1));
    post!(m, not(and(a, b)) == or(not(a), not(b)));
    
    println!("Boolean logic functions tests passed!");
}

#[test]
fn test_enhanced_modulo_operations() {
    let mut m = Model::new();
    
    let x = m.int(1, 100);
    let y = m.int(1, 50);
    let z = m.int(0, 49);
    
    // Test modulo with variables
    post!(m, x % y == z);
    
    // Test modulo with constants
    post!(m, x % int(7) == int(3));
    post!(m, int(25) % y == z);
    
    // Test modulo in complex expressions
    post!(m, (x + y) % int(10) == int(5));
    post!(m, x % (y + int(2)) <= int(8));
    
    // Test modulo with arithmetic
    post!(m, x % y + z == int(15));
    post!(m, (x % y) * int(2) == z);
    
    // Test nested modulo
    post!(m, (x % int(10)) % int(3) == int(1));
    
    println!("Enhanced modulo operations tests passed!");
}

fn main() {
    test_sum_function_support();
    test_float_constants_math_functions();
    test_boolean_logic_functions();
    test_enhanced_modulo_operations();
    test_comprehensive_new_functionality();
    
    println!("All constraint macro tests completed successfully!");
}

#[test]
fn test_comprehensive_new_functionality() {
    let mut m = Model::new();
    
    // Create test variables
    let nums = vec![m.int(1, 10), m.int(1, 10), m.int(1, 10)];
    let floats = vec![m.float(1.0, 10.0), m.float(1.0, 10.0)];
    let bools = vec![m.int(0, 1), m.int(0, 1), m.int(0, 1)];
    
    // Test combination of new features using clean syntax
    post!(m, sum(nums.clone()) + abs(floats[0]) <= float(25.5));
    post!(m, or(and(bools[0], bools[1]), not(bools[2])) == int(1));
    post!(m, sum(nums.clone()) % int(7) == int(3));
    post!(m, min(floats[0], float(5.5)) + max(floats[1], float(2.5)) >= float(8.0));
    
    // Test complex nested expressions
    post!(m, sum([nums[0], nums[1]]) * int(2) + nums[2] % int(3) == int(15));
    post!(m, and(nums[0] >= int(5), or(bools[0], bools[1])) == int(1));
    
    // Test float constants in complex expressions
    post!(m, abs(floats[0] - float(5.0)) <= max(floats[1], float(3.5)));
    
    println!("Comprehensive new functionality tests passed!");
}

fn main() {
    test_sum_function_support();
    test_float_constants_math_functions();
    test_boolean_logic_functions();
    test_enhanced_modulo_operations();
    test_boolean_function_style_operations();
    test_comprehensive_new_functionality();
    
    println!("All constraint macro tests completed successfully!");
}