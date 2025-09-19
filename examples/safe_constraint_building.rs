use cspsolver::prelude::*;

#[derive(Debug)]
enum ConstraintError {
    UnknownOperator(String),
    InvalidValue(i32),
}

/// Build a constraint safely without panicking
fn build_constraint(var: VarId, op: &str, value: i32) -> Result<Constraint, ConstraintError> {
    match op {
        "eq" => Ok(var.eq(int(value))),
        "ne" => Ok(var.ne(int(value))),
        "gt" => Ok(var.gt(int(value))),
        "ge" => Ok(var.ge(int(value))),
        "lt" => Ok(var.lt(int(value))),
        "le" => Ok(var.le(int(value))),
        _ => Err(ConstraintError::UnknownOperator(op.to_string()))
    }
}

fn main() {
    println!("🔧 Safe Constraint Building Demo");
    println!("===============================");
    
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    let z = model.int(1, 10);
    
    // Example: Build constraints from runtime data
    let constraint_specs = vec![
        ("x", "ge", 5),     // Valid
        ("y", "le", 8),     // Valid
        ("z", "eq", 3),     // Valid
        ("x", "invalid", 1), // Invalid operator - should not panic!
    ];
    
    println!("\n📋 Building constraints from data:");
    let mut successful_constraints = Vec::new();
    let mut errors = Vec::new();
    
    for (var_name, op, value) in constraint_specs {
        let var = match var_name {
            "x" => x,
            "y" => y, 
            "z" => z,
            _ => continue,
        };
        
        match build_constraint(var, op, value) {
            Ok(constraint) => {
                println!("  ✅ {var_name} {op} {value} → constraint created");
                successful_constraints.push(constraint);
            }
            Err(e) => {
                println!("  ❌ {var_name} {op} {value} → {e:?}");
                errors.push(e);
            }
        }
    }
    
    // Post all successful constraints
    for constraint in successful_constraints {
        model.post(constraint);
    }
    
    // Report summary
    println!("\n📊 Summary:");
    println!("  - Successful constraints: {}", 3);
    println!("  - Failed constraints: {}", errors.len());
    println!("  - No panics! Application continues gracefully.");
    
    // Solve the model
    println!("\n🔍 Solving model...");
    match model.solve() {
        Ok(solution) => {
            println!("✅ Solution found!");
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            let z_val = solution.get_int(z);
            
            println!("  x = {x_val}, y = {y_val}, z = {z_val}");
            
            // Verify constraints are satisfied
            assert!(x_val >= 5);
            assert!(y_val <= 8);
            assert!(z_val == 3);
            
            println!("✅ All constraints satisfied!");
        }
        Err(e) => {
            println!("❌ No solution found: {e:?}");
        }
    }
    
    println!("\n🎉 Demo complete - no panics, graceful error handling!");
}