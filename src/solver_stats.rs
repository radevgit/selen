#![allow(dead_code)]




#[derive(Debug)]
pub struct SolverStats {}

impl SolverStats {
    pub fn new() -> Self {
        SolverStats {  }
    }
}

#[cfg(test)]
mod test_solver_stats {
    use super::*;


    #[test]
    fn test_debug() {
        let stat = SolverStats::new();
        assert_eq!(format!("{:?}", stat), "SolverStats");
    }

}