use cspsolver::utils::{almost_equal_as_int, float_equal};

fn main() {
    let a = 1.0f32;
    let next1 = f32::from_bits(a.to_bits() + 1);
    let next2 = f32::from_bits(a.to_bits() + 2);
    
    println!("a = {}, bits = {:032b}", a, a.to_bits());
    println!("next1 = {}, bits = {:032b}", next1, next1.to_bits());
    println!("next2 = {}, bits = {:032b}", next2, next2.to_bits());
    
    println!("almost_equal_as_int(a, next1, 1) = {}", almost_equal_as_int(a, next1, 1));
    println!("almost_equal_as_int(a, next2, 1) = {}", almost_equal_as_int(a, next2, 1));
    println!("almost_equal_as_int(a, next2, 2) = {}", almost_equal_as_int(a, next2, 2));
    
    println!("float_equal(a, next1) = {}", float_equal(a, next1));
    println!("float_equal(a, next2) = {}", float_equal(a, next2));
    
    // Check the ULP difference calculation
    let a_i = a.to_bits() as i32;
    let next2_i = next2.to_bits() as i32;
    println!("ULP difference: {}", (a_i - next2_i).abs());
}
