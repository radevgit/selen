use cspsolver::prelude::*;

#[test]
fn debug_step_alignment() {
    let mut model = Model::default();
    let step_size = model.float_step_size();
    println!("Step size: {}", step_size);
    
    // Check if 5.5 is properly aligned with the step grid
    let x = model.float(1.0, 10.0);
    
    // The interval should be [1.0, 10.0] with step size 1e-6
    // Let's see what values are actually on the grid
    let val_5_5 = 5.5;
    let steps_from_min = (val_5_5 - 1.0) / step_size;
    println!("5.5 is {} steps from min 1.0", steps_from_min);
    println!("Steps from min (rounded): {}", steps_from_min.round());
    
    let aligned_5_5 = 1.0 + steps_from_min.round() * step_size;
    println!("5.5 aligned to grid: {}", aligned_5_5);
    println!("Difference: {}", (aligned_5_5 - 5.5).abs());
    
    // Test what prev(5.5) should be
    let prev_5_5 = aligned_5_5 - step_size;
    println!("prev(5.5) = {}", prev_5_5);
    
    // And what next(prev(5.5)) is
    let next_prev_5_5 = prev_5_5 + step_size;
    println!("next(prev(5.5)) = {}", next_prev_5_5);
    println!("Should equal 5.5: {}", (next_prev_5_5 - aligned_5_5).abs() < 1e-15);
}
