use cspsolver::props::{Prune, Propagate};
use cspsolver::props::count::Count;
use cspsolver::vars::{Val, Vars};
use cspsolver::views::Context;
use std::rc::Rc;

fn main() {
    println!("Testing Count trait object dispatch...");
    
    // Create a Count constraint
    let mut vars = Vars::new();
    let v1 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
    let v2 = vars.new_var_with_bounds(Val::int(1), Val::int(3));
    let count_var = vars.new_var_with_bounds(Val::int(1), Val::int(1));
    
    let count = Count::new(vec![v1, v2], Val::int(1), count_var);
    
    // Store as trait object
    let trait_object: Box<dyn Prune> = Box::new(count);
    let shared_trait_object = Rc::new(trait_object);
    
    // Test calling prune through trait object
    let mut events = Vec::new();
    let mut ctx = Context::new(&mut vars, &mut events);
    
    println!("Calling prune through trait object...");
    let result = shared_trait_object.as_ref().prune(&mut ctx);
    println!("Result: {:?}", result.is_some());
}