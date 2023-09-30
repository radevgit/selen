#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use crate::vari::VarI;


pub type VarId = usize;
pub type VarRef = Rc<RefCell<Var>>;

// Enum of all variabletypes
pub enum Var {
    VarI(VarI),
    VarF,
}


#[cfg(test)]
mod variable_tests {

}