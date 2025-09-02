fn main() {
    let a = 0.0f32;
    let b = -1.4e-45f32; // approximately what we get
    
    println!("a = {} (bits: {:032b})", a, a.to_bits());
    println!("b = {} (bits: {:032b})", b, b.to_bits());
    
    let a_i = a.to_bits() as i32;
    let b_i = b.to_bits() as i32;
    println!("a_i = {} (raw)", a_i);
    println!("b_i = {} (raw)", b_i);
    
    let two_comp = 0x8000_0000_u32 as i32;
    let mut a_transformed = a_i;
    let mut b_transformed = b_i;
    
    if a_i < 0 {
        a_transformed = two_comp - a_i;
    }
    if b_i < 0 {
        b_transformed = two_comp - b_i;
    }
    
    println!("a_transformed = {}", a_transformed);
    println!("b_transformed = {}", b_transformed);
    println!("diff = {}", (a_transformed - b_transformed).abs());
    println!("FLOAT_INT_EPS = 10");
    println!("Is diff <= 10? {}", (a_transformed - b_transformed).abs() <= 10);
}
