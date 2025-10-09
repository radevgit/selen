//! Basis management for Simplex method
//!
//! Manages the basic and non-basic variables during Simplex iterations.
//! The basis defines which variables are currently in the solution.

use super::matrix::Matrix;
use super::lu::LuDecomposition;
use super::types::{LpError, LpConfig};

/// Basis manager for Simplex method
///
/// In standard form LP: maximize c^T x subject to Ax = b, x >= 0
/// The basis B is a subset of columns of A that forms an invertible matrix.
/// Basic variables: x_B = B^(-1) b
/// Non-basic variables: x_N = 0
#[derive(Debug, Clone)]
pub struct Basis {
    /// Indices of basic variables (length = number of constraints)
    pub basic: Vec<usize>,
    
    /// Indices of non-basic variables
    pub nonbasic: Vec<usize>,
    
    /// LU decomposition of current basis matrix B
    /// Used for efficient solving of B x = b
    pub lu: Option<LuDecomposition>,
}

impl Basis {
    /// Create initial basis from identity columns (slack variables)
    ///
    /// For standard form with m constraints and n variables:
    /// - Basic: variables [n-m, n-m+1, ..., n-1] (last m variables, typically slacks)
    /// - Non-basic: variables [0, 1, ..., n-m-1] (first n-m variables)
    pub fn initial(n_vars: usize, n_constraints: usize) -> Self {
        debug_assert!(n_vars >= n_constraints, "Need at least as many variables as constraints");
        
        let basic: Vec<usize> = (n_vars - n_constraints..n_vars).collect();
        let nonbasic: Vec<usize> = (0..n_vars - n_constraints).collect();
        
        Self {
            basic,
            nonbasic,
            lu: None,
        }
    }
    
    /// Create basis from explicit indices
    pub fn from_indices(basic: Vec<usize>, nonbasic: Vec<usize>) -> Self {
        Self {
            basic,
            nonbasic,
            lu: None,
        }
    }
    
    /// Update LU decomposition for current basis matrix
    ///
    /// Extracts columns corresponding to basic variables and computes LU factorization
    pub fn factorize(&mut self, a: &Matrix, config: &LpConfig) -> Result<(), LpError> {
        let basis_matrix = self.extract_basis_matrix(a);
        self.lu = Some(LuDecomposition::decompose(&basis_matrix, config.feasibility_tol)?);
        Ok(())
    }
    
    /// Extract basis matrix B from constraint matrix A
    ///
    /// B consists of columns of A corresponding to basic variables
    fn extract_basis_matrix(&self, a: &Matrix) -> Matrix {
        let m = self.basic.len();
        let mut b = Matrix::zeros(m, m);
        
        for (j, &col_idx) in self.basic.iter().enumerate() {
            for i in 0..m {
                b.set(i, j, a.get(i, col_idx));
            }
        }
        b
    }
    
    /// Solve B x_B = b to get basic variable values
    ///
    /// Returns values for all variables (basic and non-basic)
    pub fn solve_basic(&self, b: &[f64]) -> Result<Vec<f64>, LpError> {
        let lu = self.lu.as_ref().ok_or(LpError::NumericalInstability)?;
        
        // Solve for basic variables: x_B = B^(-1) b
        let x_basic = lu.solve(b)?;
        
        // Create full solution vector (non-basic variables are 0)
        let n = self.basic.len() + self.nonbasic.len();
        let mut x = vec![0.0; n];
        
        for (i, &idx) in self.basic.iter().enumerate() {
            x[idx] = x_basic[i];
        }
        
        Ok(x)
    }
    
    /// Compute reduced costs for non-basic variables
    ///
    /// Reduced cost: r_j = c_j - c_B^T B^(-1) A_j
    /// Where c_j is objective coefficient, A_j is column j of constraint matrix
    pub fn compute_reduced_costs(
        &self,
        a: &Matrix,
        c: &[f64],
    ) -> Result<Vec<f64>, LpError> {
        // Compute dual variables: y = (B^T)^(-1) c_B
        // This is equivalent to solving B^T y = c_B
        let mut c_basic = vec![0.0; self.basic.len()];
        for (i, &idx) in self.basic.iter().enumerate() {
            c_basic[i] = c[idx];
        }
        
        // Solve B^T y = c_B using LU decomposition
        // Since (B^T)^(-1) = (B^(-1))^T, we can use the LU decomposition
        let y = self.solve_transpose_system(&c_basic)?;
        
        // Compute reduced costs for non-basic variables
        let mut reduced_costs = Vec::with_capacity(self.nonbasic.len());
        
        for &j in &self.nonbasic {
            let mut r_j = c[j];
            
            // Subtract y^T A_j
            for i in 0..a.rows {
                r_j -= y[i] * a.get(i, j);
            }
            
            reduced_costs.push(r_j);
        }
        
        Ok(reduced_costs)
    }
    
    /// Solve B^T y = c for dual variables
    fn solve_transpose_system(&self, c_basic: &[f64]) -> Result<Vec<f64>, LpError> {
        let lu = self.lu.as_ref().ok_or(LpError::NumericalInstability)?;
        
        // For LU decomposition of B: B = PLU
        // We need to solve B^T y = c
        // (PLU)^T y = c
        // U^T L^T P^T y = c
        
        // This is equivalent to solving the transposed system
        // For simplicity, we extract L and U and solve directly
        let l = lu.extract_l();
        let u = lu.extract_u();
        
        // Solve U^T z = c
        let mut z = c_basic.to_vec();
        for i in 0..z.len() {
            for j in 0..i {
                z[i] -= u.get(j, i) * z[j];
            }
            z[i] /= u.get(i, i);
        }
        
        // Solve L^T P^T y = z
        let mut y = vec![0.0; z.len()];
        for i in (0..z.len()).rev() {
            let mut sum = z[i];
            for j in (i + 1)..z.len() {
                sum -= l.get(j, i) * y[j];
            }
            y[i] = sum;
        }
        
        // Apply inverse permutation
        let mut result = vec![0.0; y.len()];
        for (i, &perm_i) in lu.permutation.iter().enumerate() {
            result[perm_i] = y[i];
        }
        
        Ok(result)
    }
    
    /// Check if current solution is primal feasible
    ///
    /// All basic variables must be >= 0 (within tolerance)
    pub fn is_primal_feasible(&self, x: &[f64], tolerance: f64) -> bool {
        for &idx in &self.basic {
            if x[idx] < -tolerance {
                return false;
            }
        }
        true
    }
    
    /// Check if current solution is dual feasible (for maximization)
    ///
    /// All reduced costs for non-basic variables must be <= 0 (within tolerance)
    pub fn is_dual_feasible(&self, reduced_costs: &[f64], tolerance: f64) -> bool {
        reduced_costs.iter().all(|&r| r <= tolerance)
    }
    
    /// Find entering variable (most positive reduced cost for maximization)
    ///
    /// Returns index in nonbasic array, or None if all reduced costs <= 0
    pub fn find_entering_variable(&self, reduced_costs: &[f64]) -> Option<usize> {
        let mut best_idx = None;
        let mut best_value = 0.0;
        
        for (i, &rc) in reduced_costs.iter().enumerate() {
            if rc > best_value {
                best_value = rc;
                best_idx = Some(i);
            }
        }
        
        best_idx
    }
    
    /// Find leaving variable using minimum ratio test
    ///
    /// Returns index in basic array, or None if unbounded (no positive ratios)
    pub fn find_leaving_variable(
        &self,
        x_basic: &[f64],
        direction: &[f64],
        tolerance: f64,
    ) -> Option<usize> {
        let mut best_idx = None;
        let mut best_ratio = f64::INFINITY;
        
        for (i, (&x_i, &d_i)) in x_basic.iter().zip(direction.iter()).enumerate() {
            // Only consider positive direction (leaving basis)
            if d_i > tolerance {
                let ratio = x_i / d_i;
                if ratio >= 0.0 && ratio < best_ratio {
                    best_ratio = ratio;
                    best_idx = Some(i);
                }
            }
        }
        
        best_idx
    }
    
    /// Perform basis swap: move entering to basic, leaving to non-basic
    pub fn swap(&mut self, entering_nonbasic_idx: usize, leaving_basic_idx: usize) {
        let entering_var = self.nonbasic[entering_nonbasic_idx];
        let leaving_var = self.basic[leaving_basic_idx];
        
        self.basic[leaving_basic_idx] = entering_var;
        self.nonbasic[entering_nonbasic_idx] = leaving_var;
        
        // Invalidate LU decomposition (needs refactorization)
        self.lu = None;
    }
    
    /// Get current objective value
    pub fn objective_value(&self, c: &[f64], x: &[f64]) -> f64 {
        c.iter().zip(x.iter()).map(|(ci, xi)| ci * xi).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_initial() {
        let basis = Basis::initial(5, 2);
        
        // Basic: last 2 variables [3, 4]
        assert_eq!(basis.basic, vec![3, 4]);
        
        // Non-basic: first 3 variables [0, 1, 2]
        assert_eq!(basis.nonbasic, vec![0, 1, 2]);
        
        assert!(basis.lu.is_none());
    }
    
    #[test]
    fn test_basis_from_indices() {
        let basis = Basis::from_indices(vec![0, 2], vec![1, 3]);
        
        assert_eq!(basis.basic, vec![0, 2]);
        assert_eq!(basis.nonbasic, vec![1, 3]);
    }
    
    #[test]
    fn test_extract_basis_matrix() {
        let a = Matrix::from_rows(vec![
            vec![1.0, 2.0, 1.0, 0.0],
            vec![3.0, 4.0, 0.0, 1.0],
        ]);
        
        let basis = Basis::from_indices(vec![2, 3], vec![0, 1]);
        let b = basis.extract_basis_matrix(&a);
        
        // Columns 2 and 3 form identity matrix
        assert_eq!(b.get(0, 0), 1.0);
        assert_eq!(b.get(0, 1), 0.0);
        assert_eq!(b.get(1, 0), 0.0);
        assert_eq!(b.get(1, 1), 1.0);
    }
    
    #[test]
    fn test_factorize_and_solve() {
        let a = Matrix::from_rows(vec![
            vec![2.0, 1.0, 1.0, 0.0],
            vec![1.0, 2.0, 0.0, 1.0],
        ]);
        
        let mut basis = Basis::from_indices(vec![2, 3], vec![0, 1]);
        let config = LpConfig::default();
        
        basis.factorize(&a, &config).unwrap();
        assert!(basis.lu.is_some());
        
        // Solve with identity basis matrix
        let b_vec = vec![5.0, 6.0];
        let x = basis.solve_basic(&b_vec).unwrap();
        
        // x[2] = 5, x[3] = 6, others = 0
        assert_eq!(x.len(), 4);
        assert_eq!(x[0], 0.0);
        assert_eq!(x[1], 0.0);
        assert_eq!(x[2], 5.0);
        assert_eq!(x[3], 6.0);
    }
    
    #[test]
    fn test_primal_feasibility() {
        let basis = Basis::from_indices(vec![0, 2], vec![1, 3]);
        
        // All basic variables >= 0
        let x_feasible = vec![1.0, 0.0, 2.0, 0.0];
        assert!(basis.is_primal_feasible(&x_feasible, 1e-6));
        
        // One basic variable < 0
        let x_infeasible = vec![1.0, 0.0, -0.1, 0.0];
        assert!(!basis.is_primal_feasible(&x_infeasible, 1e-6));
        
        // Within tolerance
        let x_tolerance = vec![1.0, 0.0, -1e-7, 0.0];
        assert!(basis.is_primal_feasible(&x_tolerance, 1e-6));
    }
    
    #[test]
    fn test_dual_feasibility() {
        let basis = Basis::from_indices(vec![0, 1], vec![2, 3]);
        
        // All reduced costs <= 0
        let rc_feasible = vec![-1.0, -0.5];
        assert!(basis.is_dual_feasible(&rc_feasible, 1e-6));
        
        // One reduced cost > 0
        let rc_infeasible = vec![-1.0, 0.1];
        assert!(!basis.is_dual_feasible(&rc_infeasible, 1e-6));
        
        // Within tolerance
        let rc_tolerance = vec![-1.0, 1e-7];
        assert!(basis.is_dual_feasible(&rc_tolerance, 1e-6));
    }
    
    #[test]
    fn test_find_entering_variable() {
        let basis = Basis::from_indices(vec![0, 1], vec![2, 3, 4]);
        
        // Most positive reduced cost
        let rc = vec![1.0, 3.0, 0.5];
        let entering = basis.find_entering_variable(&rc);
        assert_eq!(entering, Some(1)); // Index 1 has value 3.0
        
        // All non-positive
        let rc_none = vec![-1.0, 0.0, -0.5];
        let entering_none = basis.find_entering_variable(&rc_none);
        assert_eq!(entering_none, None);
    }
    
    #[test]
    fn test_find_leaving_variable() {
        let basis = Basis::from_indices(vec![0, 1, 2], vec![3, 4]);
        
        let x_basic = vec![4.0, 6.0, 8.0];
        let direction = vec![2.0, 3.0, 1.0];
        
        // Minimum ratio test: 4/2=2, 6/3=2, 8/1=8
        // First minimum ratio at index 0
        let leaving = basis.find_leaving_variable(&x_basic, &direction, 1e-6);
        assert_eq!(leaving, Some(0));
        
        // No positive direction (unbounded)
        let direction_unbounded = vec![-1.0, 0.0, -2.0];
        let leaving_unbounded = basis.find_leaving_variable(&x_basic, &direction_unbounded, 1e-6);
        assert_eq!(leaving_unbounded, None);
    }
    
    #[test]
    fn test_basis_swap() {
        let mut basis = Basis::from_indices(vec![0, 2], vec![1, 3]);
        
        // Swap: entering = 1 (nonbasic[0]), leaving = 2 (basic[1])
        basis.swap(0, 1);
        
        assert_eq!(basis.basic, vec![0, 1]);
        assert_eq!(basis.nonbasic, vec![2, 3]);
        assert!(basis.lu.is_none()); // LU invalidated
    }
    
    #[test]
    fn test_objective_value() {
        let basis = Basis::from_indices(vec![0, 1], vec![2, 3]);
        let c = vec![3.0, 2.0, 1.0, 4.0];
        let x = vec![5.0, 6.0, 0.0, 0.0];
        
        let obj = basis.objective_value(&c, &x);
        // 3*5 + 2*6 + 1*0 + 4*0 = 27
        assert_eq!(obj, 27.0);
    }
    
    #[test]
    fn test_compute_reduced_costs() {
        // Simple example: maximize 3x1 + 2x2 subject to x1 + x2 <= 5
        // Standard form: maximize 3x1 + 2x2 + 0x3 subject to x1 + x2 + x3 = 5
        // A = [1 1 1], c = [3 2 0], b = [5]
        // Initial basis: x3 (slack), so basic = [2], nonbasic = [0, 1]
        
        let a = Matrix::from_rows(vec![
            vec![1.0, 1.0, 1.0],
        ]);
        let c = vec![3.0, 2.0, 0.0];
        
        let mut basis = Basis::from_indices(vec![2], vec![0, 1]);
        let config = LpConfig::default();
        basis.factorize(&a, &config).unwrap();
        
        let reduced_costs = basis.compute_reduced_costs(&a, &c).unwrap();
        
        // For initial basis with slack: reduced costs should be original costs
        // r_0 = c_0 - 0 = 3, r_1 = c_1 - 0 = 2
        assert!((reduced_costs[0] - 3.0).abs() < 1e-10);
        assert!((reduced_costs[1] - 2.0).abs() < 1e-10);
    }
}
