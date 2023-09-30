#![allow(dead_code)]




#[derive(Debug)]
pub struct SolverOptions {}

impl SolverOptions {
    pub fn new() -> Self {
        SolverOptions {  }
    }
}


#[cfg(test)]
mod test_solver_options {
    use super::*;


    #[test]
    fn test_debug() {
        let stat = SolverOptions::new();
        assert_eq!(format!("{:?}", stat), "SolverOptions");
    }

}