use cspsolver::prelude::*;

fn main() {
    let mut m = Model::default();
    let x = m.new_var_int(1, 5);
    let y = m.new_var_int(1, 5);
    
    // Try to call not_equals
    m.not_equals(x, y);
    
    println!("not_equals method called successfully!");
}
