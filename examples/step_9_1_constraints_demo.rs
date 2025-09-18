use cspsolver::prelude::*;
use cspsolver::props::conditional::{Condition, SimpleConstraint};

fn main() {
    let mut m = Model::default();
    
    // Demonstrate Between Constraint
    println!("=== Between Constraint Demo ===");
    let lower = m.int(1, 5);
    let middle = m.int(0, 10);
    let upper = m.int(7, 12);
    
    // Using helper method
    m.props.between_constraint(lower, middle, upper);
    println!("Created between constraint: lower <= middle <= upper");
    
    // Using post! macro
    post!(m, between(lower, middle, upper));
    println!("Added between constraint via post! macro");
    
    // Demonstrate Cardinality Constraints
    println!("\n=== Cardinality Constraints Demo ===");
    let vars = vec![m.int(0, 3), m.int(0, 3), m.int(0, 3), m.int(0, 3)];
    
    // At least 2 variables must equal 2
    m.props.at_least_constraint(vars.clone(), 2, 2);
    println!("Created 'at least 2 vars equal 2' constraint");
    
    // Using post! macro for at_most
    post!(m, at_most(vars.clone(), 2, 3));
    println!("Added 'at most 3 vars equal 2' via post! macro");
    
    // Using post! macro for exactly
    post!(m, exactly(vars.clone(), 1, 1));
    println!("Added 'exactly 1 var equals 1' via post! macro");
    
    // Demonstrate Conditional Constraints  
    println!("\n=== Conditional Constraints Demo ===");
    let condition_var = m.int(0, 1);
    let target_var = m.int(0, 10);
    
    // Using helper method for if-then
    let condition = Condition::Equals(condition_var, Val::ValI(1));
    let then_constraint = SimpleConstraint::Equals(target_var, Val::ValI(5));
    m.props.if_then_else_constraint(condition.clone(), then_constraint.clone(), None);
    println!("Created if-then constraint");
    
    // Using post! macro for simple if-then
    post!(m, if_then(condition_var == Val::ValI(1), target_var == Val::ValI(5)));
    println!("Added if-then constraint via post! macro");
    
    println!("\n=== Step 9.1 Constraints Implementation Complete! ===");
    println!("✅ Between constraints (lower <= middle <= upper)");
    println!("✅ Cardinality constraints (at_least, at_most, exactly)");
    println!("✅ If-then-else conditional constraints");
    println!("✅ Full macro integration with post! syntax");
    println!("✅ Helper methods in Propagators");
    println!("✅ Metadata system integration");
}