fn main() {
    let a = 0.0f32;
    let b = -1.4e-45f32;
    
    println!("a.signum() = {}", a.signum());
    println!("b.signum() = {}", b.signum());
    println!("a.signum() != b.signum() = {}", a.signum() != b.signum());
    println!("a == b = {}", a == b);
}
