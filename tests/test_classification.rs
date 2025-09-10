use cspsolver::prelude::*;
use cspsolver::optimization::classification::ProblemClassifier;

#[test]
fn test_pure_float_problem_classification() {
    let mut model = Model::default();
    let x = model.new_var_float(1.0, 10.0);
    let y = model.new_var_float(2.0, 20.0);
    model.less_than(x, y);
    
    let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
    
    match problem_type {
        cspsolver::optimization::classification::ProblemType::PureFloat { 
            float_var_count: 2, 
            has_linear_bounds_only: true 
        } => {
            // Correct classification
        },
        _ => panic!("Expected PureFloat classification, got {:?}", problem_type),
    }
    
    assert!(problem_type.can_use_efficient_float_optimization());
    assert!(!problem_type.requires_integer_search());
    
    let strategy = problem_type.strategy_description();
    assert!(strategy.contains("O(1)"));
    assert!(strategy.contains("analytical"));
}

#[test]
fn test_pure_integer_problem_classification() {
    let mut model = Model::default();
    let a = model.new_var_int(1, 10);
    let b = model.new_var_int(5, 15);
    model.not_equals(a, b);
    
    let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
    
    match problem_type {
        cspsolver::optimization::classification::ProblemType::PureInteger { 
            integer_var_count: 2 
        } => {
            // Correct classification
        },
        _ => panic!("Expected PureInteger classification, got {:?}", problem_type),
    }
    
    assert!(!problem_type.can_use_efficient_float_optimization());
    assert!(problem_type.requires_integer_search());
    
    let strategy = problem_type.strategy_description();
    assert!(strategy.contains("Binary search"));
    assert!(strategy.contains("current solver"));
}

#[test]
fn test_mixed_problem_with_coupling() {
    let mut model = Model::default();
    let int_var = model.new_var_int(1, 5);
    let float_var = model.new_var_float(1.0, 10.0);
    model.equals(int_var, float_var); // This creates coupling
    
    let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
    
    match problem_type {
        cspsolver::optimization::classification::ProblemType::MixedCoupled { 
            integer_var_count: 1, 
            float_var_count: 1, 
            coupling_strength: _ 
        } => {
            // Correct classification
        },
        _ => panic!("Expected MixedCoupled classification, got {:?}", problem_type),
    }
    
    let strategy = problem_type.strategy_description();
    assert!(strategy.contains("MINLP"));
}

#[test]
fn test_mixed_problem_expected_separable() {
    // This test shows the current conservative behavior
    // In the future, we may refine this to detect true separability
    let mut model = Model::default();
    let int_var1 = model.new_var_int(1, 5);
    let int_var2 = model.new_var_int(3, 8);
    let float_var1 = model.new_var_float(1.0, 10.0);
    let float_var2 = model.new_var_float(5.0, 15.0);
    
    // Add constraints within each type only (no cross-type coupling)
    model.not_equals(int_var1, int_var2);
    model.less_than(float_var1, float_var2);
    
    let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
    
    // Currently this is classified as MixedCoupled due to conservative heuristic
    // In the future, improved constraint analysis could detect this as MixedSeparable
    match problem_type {
        cspsolver::optimization::classification::ProblemType::MixedCoupled { 
            integer_var_count: 2, 
            float_var_count: 2, 
            coupling_strength: _ 
        } => {
            // Current conservative classification
        },
        cspsolver::optimization::classification::ProblemType::MixedSeparable { 
            integer_var_count: 2, 
            float_var_count: 2 
        } => {
            // Future improved classification
        },
        _ => panic!("Expected MixedCoupled or MixedSeparable classification, got {:?}", problem_type),
    }
}

#[test]
fn test_empty_model_classification() {
    let model = Model::default();
    let problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
    
    // Empty model should be classified as PureInteger with 0 variables
    match problem_type {
        cspsolver::optimization::classification::ProblemType::PureInteger { 
            integer_var_count: 0 
        } => {
            // Correct classification for empty model
        },
        _ => panic!("Expected PureInteger(0) classification for empty model, got {:?}", problem_type),
    }
}

#[test]
fn test_single_variable_problems() {
    // Single float variable
    let mut model1 = Model::default();
    let _x = model1.new_var_float(0.0, 100.0);
    let problem_type1 = ProblemClassifier::classify(model1.get_vars(), model1.get_props());
    
    match problem_type1 {
        cspsolver::optimization::classification::ProblemType::PureFloat { 
            float_var_count: 1, 
            has_linear_bounds_only: true 
        } => {
            // Correct
        },
        _ => panic!("Expected PureFloat(1) classification, got {:?}", problem_type1),
    }
    
    // Single integer variable
    let mut model2 = Model::default();
    let _a = model2.new_var_int(1, 100);
    let problem_type2 = ProblemClassifier::classify(model2.get_vars(), model2.get_props());
    
    match problem_type2 {
        cspsolver::optimization::classification::ProblemType::PureInteger { 
            integer_var_count: 1 
        } => {
            // Correct
        },
        _ => panic!("Expected PureInteger(1) classification, got {:?}", problem_type2),
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_classification_performance() {
        // Test that classification is fast even for larger models
        let mut model = Model::default();
        let mut var_ids = Vec::new();
        
        // Create a model with many variables
        for i in 0..100 {
            if i % 2 == 0 {
                let var_id = model.new_var_int(1, 100);
                var_ids.push(var_id);
            } else {
                let var_id = model.new_var_float(0.0, 100.0);
                var_ids.push(var_id);
            }
        }
        
        // Add some constraints
        for i in 0..50 {
            let var1 = var_ids[i * 2];
            let var2 = var_ids[i * 2 + 1];
            model.less_than(var1, var2);
        }
        
        // Classification should be fast
        let start = std::time::Instant::now();
        let _problem_type = ProblemClassifier::classify(model.get_vars(), model.get_props());
        let duration = start.elapsed();
        
        // Should complete in well under 1ms for 100 variables
        assert!(duration.as_millis() < 10, "Classification took too long: {:?}", duration);
    }
}
