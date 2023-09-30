#![allow(dead_code)]




pub type ConId =  usize;

// Trait for all constraints
pub trait Constraint {
    fn id(&self) -> ConId;
    fn set_active(&mut self, b: bool);
    fn is_active(&self) -> bool;
    
    fn post(&mut self);
    fn propagate(&mut self) -> bool;

    fn on_fix(&self); // propagate on fix
    fn on_domain_change(&self); // propagate on domain change
    fn on_bound_change(&self); // propagate on bound change
}


#[cfg(test)]
mod constraint_tests {

}