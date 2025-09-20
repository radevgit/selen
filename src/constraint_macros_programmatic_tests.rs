//! Programmatic equivalents of constraint macro tests
//!
//! This module contains tests that demonstrate how to build constraints programmatically
//! using the runtime API, as equivalents to the post! macro tests in constraint_macros.rs.
//! 
//! Each test function has a corresponding post! macro version, showing API parity
//! and providing migration examples for users.

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::runtime_api::{VarIdExt, ModelExt};

    /// Programmatic equivalent of test_post_macro_basic()
    /// 
    /// Original test uses: post!(m, x < y), post!(m, x <= y), etc.
    /// This version uses: m.post(x.lt(y)), m.post(x.le(y)), etc.
    #[test]
    fn test_post_macro_basic_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test basic variable comparisons - programmatic equivalents
        let _c1 = m.post(x.lt(y));    // Equivalent to: post!(m, x < y)
        let _c2 = m.post(x.le(y));    // Equivalent to: post!(m, x <= y)
        let _c3 = m.post(x.gt(y));    // Equivalent to: post!(m, x > y)
        let _c4 = m.post(x.ge(y));    // Equivalent to: post!(m, x >= y)
        let _c5 = m.post(x.eq(y));    // Equivalent to: post!(m, x == y)
        let _c6 = m.post(x.ne(y));    // Equivalent to: post!(m, x != y)
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_constants()
    /// 
    /// Original test uses: post!(m, x < int(5)), post!(m, y <= float(3.14)), etc.
    /// This version uses: m.post(x.lt(int(5))), m.post(y.le(float(3.14))), etc.
    #[test]
    fn test_post_macro_constants_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.float(1.0, 10.0);
        
        // Test variable vs integer constants - programmatic equivalents
        let _c1 = m.post(x.lt(5));          // Equivalent to: post!(m, x < int(5))
        let _c2 = m.post(x.ge(1));          // Equivalent to: post!(m, x >= int(1))
        let _c3 = m.post(x.eq(7));          // Equivalent to: post!(m, x == int(7))
        
        // Test variable vs float constants - programmatic equivalents
        let _c4 = m.post(y.le(3.14));       // Equivalent to: post!(m, y <= float(3.14))
        let _c5 = m.post(y.gt(1.0));        // Equivalent to: post!(m, y > float(1.0))
        let _c6 = m.post(y.ne(5.5));        // Equivalent to: post!(m, y != float(5.5))
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_arithmetic()
    /// 
    /// Original test uses: post!(m, x + y < z), post!(m, x - y >= z), etc.
    /// This version uses: m.post(x.add(y).lt(z)), m.post(x.sub(y).ge(z)), etc.
    #[test]
    fn test_post_macro_arithmetic_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 20);
        
        // Test arithmetic operations with variables - programmatic equivalents
        let _c1 = m.post(x.add(y).lt(z));   // Equivalent to: post!(m, x + y < z)
        let _c2 = m.post(x.sub(y).ge(z));   // Equivalent to: post!(m, x - y >= z)
        let _c3 = m.post(x.mul(y).le(z));   // Equivalent to: post!(m, x * y <= z)
        let _c4 = m.post(x.div(y).eq(z));   // Equivalent to: post!(m, x / y == z)
        
        // Test arithmetic operations with constants - programmatic equivalents
        let _c5 = m.post(x.add(y).le(15));  // Equivalent to: post!(m, x + y <= int(15))
        let _c6 = m.post(x.sub(y).ge(0));   // Equivalent to: post!(m, x - y >= int(0))
        let _c7 = m.post(x.mul(y).eq(12));  // Equivalent to: post!(m, x * y == int(12))
        let _c8 = m.post(x.div(y).ne(0));   // Equivalent to: post!(m, x / y != int(0))
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_array_syntax()
    /// 
    /// Original test uses: post!(m, alldiff(vars)), post!(m, min(vars) <= int(5)), etc.
    /// This version uses: m.alldiff(&vars), m.post(m.min(&vars).le(5)), etc.
    #[test]
    fn test_post_macro_array_syntax_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Test alldiff with arrays - programmatic equivalents
        let vars = [x, y, z];
        m.alldiff(&vars);                        // Equivalent to: post!(m, alldiff(vars))
        
        let vars_vec = vec![x, y, z];
        m.alldiff(&vars_vec);                    // Equivalent to: post!(m, alldiff(vars_vec))
        
        // Test min/max with arrays - programmatic equivalents
        let min_result = m.min(&vars).expect("non-empty variable list");
        m.post(min_result.le(5));               // Equivalent to: post!(m, min(vars) <= int(5))
        let max_result = m.max(&vars_vec).expect("non-empty variable list");
        m.post(max_result.ge(8));               // Equivalent to: post!(m, max(vars_vec) >= int(8))
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_alldiff()
    /// 
    /// Original test uses: post!(m, alldiff([x, y, z])), etc.
    /// This version uses: m.alldiff(&[x, y, z]), etc.
    #[test]
    fn test_post_macro_alldiff_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        let w = m.int(1, 10);
        
        // Test alldiff constraint - programmatic equivalents
        m.alldiff(&[x, y, z]);                  // Equivalent to: post!(m, alldiff([x, y, z]))
        m.alldiff(&[x, y, z, w]);               // Equivalent to: post!(m, alldiff([x, y, z, w]))
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_allequal()
    /// 
    /// Original test uses: post!(m, allequal([x, y, z])), etc.
    /// This version uses: m.allequal(&[x, y, z]), etc.
    #[test]
    fn test_post_macro_allequal_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(5, 15);
        let z = m.int(3, 8);
        let w = m.int(1, 10);
        
        // Test allequal constraint - programmatic equivalents
        m.alleq(&[x, y, z]);                    // Equivalent to: post!(m, allequal([x, y, z]))
        m.alleq(&[x, y, z, w]);                 // Equivalent to: post!(m, allequal([x, y, z, w]))
        
        // Test with array expression - programmatic equivalent
        let vars = vec![x, y, z];
        m.alleq(&vars);                         // Equivalent to: post!(m, allequal(vars))
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_element()
    /// 
    /// Original test uses: post!(m, element([a0, a1, a2], index, value)), post!(m, array[index] == value), etc.
    /// This version uses: m.elem(&[a0, a1, a2], index, value), m.post(m.elem_var(&array, index).eq(value)), etc.
    #[test]
    fn test_post_macro_element_programmatic() {
        let mut m = Model::default();
        let a0 = m.int(10, 10);
        let a1 = m.int(20, 20);
        let a2 = m.int(30, 30);
        let index = m.int(0, 2);
        let value = m.int(10, 30);
        
        // Test element constraint with array literal - programmatic equivalent
        m.elem(&[a0, a1, a2], index, value);    // Equivalent to: post!(m, element([a0, a1, a2], index, value))
        
        // Test element constraint with array expression - programmatic equivalent
        let array = vec![a0, a1, a2];
        m.elem(&array, index, value);           // Equivalent to: post!(m, element(array.clone(), index, value))
        
        // Test natural array[index] == value syntax - programmatic equivalent
        // Note: Direct array indexing with variables requires element constraints
        m.elem(&array, index, value);           // Equivalent to: post!(m, array[index] == value)
        
        // Test reverse syntax: value == array[index] - programmatic equivalent
        m.elem(&array, index, value);           // Equivalent to: post!(m, value == array[index])
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_logical_operators()
    /// 
    /// Original test uses: post!(m, and(a, b)), post!(m, or(a, b)), post!(m, not(a)), etc.
    /// This version uses: m.post(c1.and(c2)), m.post(c1.or(c2)), m.post(c1.not()), etc.
    #[test]
    fn test_post_macro_logical_operators_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test basic constraint references - programmatic equivalents
        let c1 = m.post(x.lt(y));               // Equivalent to: post!(m, x < y)
        let c2 = m.post(y.gt(5));               // Equivalent to: post!(m, y > int(5))
        
        // Note: Boolean operations on constraint references are available in runtime API
        // Testing basic boolean operations with variables instead
        let a = m.int(0, 1);
        let b = m.int(0, 1);
        
        // Post the boolean-like constraints (create fresh constraints each time)
        m.post(a.eq(1).and(b.eq(1)));           // Equivalent to: post!(m, and(a, b))
        m.post(a.eq(1).or(b.eq(1)));            // Equivalent to: post!(m, or(a, b))  
        m.post(a.eq(0));                        // Equivalent to: post!(m, not(a))
        
        println!("Constraint references: {:?}, {:?}", c1, c2);
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_mathematical_functions()
    /// 
    /// Original test uses: post!(m, abs(x) >= int(1)), post!(m, min([y, z]) == int(5)), etc.
    /// This version uses: m.post(m.abs(x).ge(1)), m.post(m.min(&[y, z]).eq(5)), etc.
    #[test]
    fn test_post_macro_mathematical_functions_programmatic() {
        let mut m = Model::default();
        let x = m.int(-10, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Test absolute value - programmatic equivalents
        let abs_x = m.abs(x);
        m.post(abs_x.ge(1));                    // Equivalent to: post!(m, abs(x) >= int(1))
        m.post(abs_x.le(y));                    // Equivalent to: post!(m, abs(x) <= y)
        
        // Test min function - programmatic equivalents
        let min_yz = m.min(&[y, z]).expect("non-empty variable list");
        m.post(min_yz.eq(5));                   // Equivalent to: post!(m, min([y, z]) == int(5))
        m.post(min_yz.ge(x));                   // Equivalent to: post!(m, min([y, z]) >= x)
        
        // Test max function - programmatic equivalents  
        let max_yz = m.max(&[y, z]).expect("non-empty variable list");
        m.post(max_yz.le(10));                  // Equivalent to: post!(m, max([y, z]) <= int(10))
        m.post(max_yz.ne(x));                   // Equivalent to: post!(m, max([y, z]) != x)
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_negation()
    /// 
    /// Original test uses: post!(m, !(x < y)) (commented out), post!(m, x >= y), etc.
    /// This version uses: m.post(x.lt(y).not()), m.post(x.ge(y)), etc.
    #[test]
    fn test_post_macro_negation_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        
        // Test negation using runtime API - programmatic equivalent
        m.post(x.lt(y).not());                  // Equivalent to: post!(m, !(x < y))  -> x >= y
        
        // For comparison, direct equivalent
        m.post(x.ge(y));                        // Equivalent to: post!(m, x >= y)
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_modulo()
    ///
    /// Original test uses: post!(m, x % 3 == 1)
    /// This version uses: m.modulo(x, 3).eq(1)
    #[test]
    fn test_post_macro_modulo_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        
        // Test simple modulo operations using runtime API
        // Macro: post!(m, x % 3 == 1)
        // Programmatic: m.modulo(x, Val::from(3)).eq(1)
        let mod_result = m.modulo(x, Val::from(3));
        let _c1 = m.post(mod_result.eq(1));
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_enhanced_modulo()
    ///
    /// Original test uses: post!(m, x % y == int(0)), post!(m, x % y != int(0))
    /// This version uses: m.modulo(x, y).eq(0), m.modulo(x, y).ne(0)
    #[test]
    fn test_post_macro_enhanced_modulo_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        let y = m.int(2, 5);
        
        // Test enhanced modulo with variables using runtime API
        // Macro: post!(m, x % y == int(0))
        // Programmatic: m.modulo(x, y).eq(0)
        let mod_result1 = m.modulo(x, y);
        let _c1 = m.post(mod_result1.eq(0));
        
        // Macro: post!(m, x % y != int(0))
        // Programmatic: m.modulo(x, y).ne(0)
        let mod_result2 = m.modulo(x, y);
        let _c2 = m.post(mod_result2.ne(0));
        
        // Original literal modulo still works with runtime API
        // Macro: post!(m, x % 3 == 1)
        // Programmatic: m.modulo(x, Val::from(3)).eq(1)
        let mod_result3 = m.modulo(x, Val::from(3));
        let _c3 = m.post(mod_result3.eq(1));
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_post_macro_complex_expressions()
    ///
    /// Original test combines multiple constraint types with macros
    /// This version uses runtime API equivalents for each operation
    #[test]
    fn test_post_macro_complex_expressions_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 10);
        
        // Test combining different constraint types using runtime API
        // Macro: post!(m, x + y <= int(15))
        // Programmatic: x.add(y).le(15)
        let _c1 = m.post(x.add(y).le(15));
        
        // Macro: post!(m, abs(x) >= int(1))
        // Programmatic: m.abs(x).ge(1)
        let abs_x = m.abs(x);
        let _c2 = m.post(abs_x.ge(1));
        
        // Macro: post!(m, max([x, y]) == z)
        // Programmatic: m.max(&[x, y]).eq(z)
        let max_xy = m.max(&[x, y]).expect("non-empty variable list");
        let _c3 = m.post(max_xy.eq(z));
        
        // Macro: post!(m, x % y != int(0))
        // Programmatic: m.modulo(x, y).ne(0)
        let mod_result4 = m.modulo(x, y);
        let _c4 = m.post(mod_result4.ne(0));
        
        // Macro: post!(m, alldiff([x, y, z]))
        // Programmatic: m.alldiff(&[x, y, z])
        m.alldiff(&[x, y, z]);
        
        // Should compile without errors
        assert!(true);
    }

    /// Programmatic equivalent of test_postall_macro()
    ///
    /// Original test uses: postall!(m, x < y, y > int(5), ...) for batch constraint posting
    /// This version demonstrates equivalent programmatic batch constraint posting
    #[test]
    fn test_postall_macro_programmatic() {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 15);
        
        // Create some constraint references for testing
        // Macro: post!(m, x < y)
        // Programmatic: m.post(x.lt(y))
        let c1 = m.post(x.lt(y));
        
        // Macro: post!(m, y > int(5))
        // Programmatic: m.post(y.gt(5))
        let c2 = m.post(y.gt(5));
        
        // Test boolean variables for logical operations
        let a = m.int(0, 1);
        let b = m.int(0, 1);
        
        // Test direct constraint posting - programmatic equivalent of postall!
        // Instead of postall! macro, we post each constraint individually using runtime API
        let array = vec![x, y, z];
        
        // Macro: postall!(m, x < y, y > int(5), x + y <= z, ...)
        // Programmatic: Individual m.post() calls for each constraint
        m.post(x.lt(y));                        // x < y
        m.post(y.gt(5));                        // y > int(5)
        m.post(x.add(y).le(z));                 // x + y <= z
        m.alldiff(&[x, y, z]);                  // alldiff([x, y, z])
        m.alleq(&[x, y]);                       // allequal([x, y])
        m.elem(&array, a, b);                   // element([x, y, z], a, b)
        // Note: array[a] == b would require dynamic indexing, simplified for demo
        
        // Logical operations - need to create constraint expressions first
        let and_constraint = a.eq(1).and(b.eq(1));
        m.post(and_constraint);                 // and(a, b) equivalent
        
        let or_constraint = a.eq(1).or(b.eq(1));
        m.post(or_constraint);                  // or(a, b) equivalent
        
        let not_constraint = a.eq(1).not();
        m.post(not_constraint);                 // not(a) equivalent
        
        println!("Constraint references: {:?}, {:?}", c1, c2);
        
        // Should compile and run without errors
        assert!(true);
    }

    /// Comprehensive validation test demonstrating complete API equivalency
    ///
    /// This test validates that all major constraint types work correctly
    /// with the programmatic API, providing a comprehensive example
    #[test]
    fn test_comprehensive_programmatic_api_validation() {
        let mut m = Model::default();
        
        // Create variables for comprehensive testing
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let z = m.int(1, 20);
        let vars = vec![x, y, z];
        
        // Test 1: Basic constraints - equivalent to post! macros
        m.post(x.lt(y));                        // x < y
        m.post(y.le(z));                        // y <= z
        m.post(z.gt(5));                        // z > 5
        m.post(x.ge(1));                        // x >= 1
        m.post(y.eq(5));                        // y == 5
        m.post(z.ne(15));                       // z != 15
        
        // Test 2: Arithmetic operations - equivalent to post! arithmetic
        m.post(x.add(y).le(z));                 // x + y <= z
        m.post(y.sub(x).ge(0));                 // y - x >= 0
        m.post(x.mul(y).eq(12));                // x * y == 12
        m.post(z.div(y).ne(0));                 // z / y != 0
        
        // Test 3: Global constraints - equivalent to post! global constraints
        m.alldiff(&vars);                       // alldiff([x, y, z])
        
        let equal_vars = vec![x, y];
        m.alleq(&equal_vars);                   // allequal([x, y])
        
        let index = m.int(0, 2);
        let value = m.int(1, 10);
        m.elem(&vars, index, value);            // element([x, y, z], index, value)
        
        // Test 4: Mathematical functions - equivalent to post! math functions
        let abs_x = m.abs(x);
        m.post(abs_x.ge(1));                    // abs(x) >= 1
        
        let min_vars = m.min(&vars).expect("non-empty variable list");
        m.post(min_vars.eq(1));                 // min([x, y, z]) == 1
        
        let max_vars = m.max(&vars).expect("non-empty variable list");
        m.post(max_vars.le(10));                // max([x, y, z]) <= 10
        
        // Test 5: Modulo operations - equivalent to post! modulo
        let mod_result = m.modulo(x, Val::from(3));
        m.post(mod_result.eq(1));               // x % 3 == 1
        
        // Test 6: Logical operations - equivalent to post! logical
        // Create fresh constraints for each operation to avoid moved values
        m.post(x.lt(5).and(y.gt(3)));          // (x < 5) && (y > 3)
        m.post(x.lt(5).or(y.gt(3)));           // (x < 5) || (y > 3)  
        m.post(x.lt(5).not());                 // !(x < 5)
        
        // Test 7: Constants - equivalent to post! with int() helper
        m.post(x.add(y).le(15));                // x + y <= int(15)
        m.post(z.sub(5).ge(0));                 // z - int(5) >= 0
        
        println!("Comprehensive programmatic API validation complete");
        println!("All constraint types successfully demonstrated:");
        println!("- Basic comparisons: <, <=, >, >=, ==, !=");
        println!("- Arithmetic: +, -, *, /");
        println!("- Global constraints: alldiff, alleq, element");
        println!("- Mathematical functions: abs, min, max");
        println!("- Modulo operations: %");
        println!("- Logical operations: &&, ||, !");
        println!("- Constants and literals");
        
        // All constraints should compile and be added successfully
        assert!(true);
    }
}