//! LU decomposition with partial pivoting
//!
//! Provides LU factorization needed for solving linear systems in the Simplex method.
//! Uses partial pivoting for numerical stability.

use super::matrix::Matrix;
use super::types::LpError;

/// LU decomposition: PA = LU where P is a permutation matrix
#[derive(Debug, Clone)]
pub struct LuDecomposition {
    /// Combined L and U matrices (L below diagonal, U on and above diagonal)
    pub lu: Matrix,
    
    /// Row permutation from partial pivoting
    pub permutation: Vec<usize>,
    
    /// Sign of permutation (+1 or -1), used for determinant calculation
    pub sign: i32,
}

impl LuDecomposition {
    /// Compute LU decomposition with partial pivoting
    ///
    /// Returns LpError::SingularBasis if matrix is singular (within tolerance)
    pub fn decompose(matrix: &Matrix, tolerance: f64) -> Result<Self, LpError> {
        if matrix.rows != matrix.cols {
            return Err(LpError::NumericalInstability);
        }
        
        let n = matrix.rows;
        let mut lu = matrix.clone();
        let mut permutation: Vec<usize> = (0..n).collect();
        let mut sign = 1;
        
        // Gaussian elimination with partial pivoting
        for k in 0..n {
            // Find pivot (largest absolute value in column k, from row k onwards)
            let mut pivot_row = k;
            let mut pivot_value = lu.get(k, k).abs();
            
            for i in (k + 1)..n {
                let val = lu.get(i, k).abs();
                if val > pivot_value {
                    pivot_row = i;
                    pivot_value = val;
                }
            }
            
            // Check for singularity
            if pivot_value < tolerance {
                return Err(LpError::SingularBasis);
            }
            
            // Swap rows if needed
            if pivot_row != k {
                lu.swap_rows(k, pivot_row);
                permutation.swap(k, pivot_row);
                sign = -sign;
            }
            
            // Eliminate below diagonal
            let pivot = lu.get(k, k);
            for i in (k + 1)..n {
                let factor = lu.get(i, k) / pivot;
                lu.set(i, k, factor); // Store multiplier in L part
                
                // Update row i
                for j in (k + 1)..n {
                    let val = lu.get(i, j) - factor * lu.get(k, j);
                    lu.set(i, j, val);
                }
            }
        }
        
        Ok(Self {
            lu,
            permutation,
            sign,
        })
    }
    
    /// Solve linear system Ax = b using the LU decomposition
    ///
    /// Uses forward substitution (Ly = Pb) then backward substitution (Ux = y)
    pub fn solve(&self, b: &[f64]) -> Result<Vec<f64>, LpError> {
        let n = self.lu.rows;
        
        if b.len() != n {
            return Err(LpError::NumericalInstability);
        }
        
        // Apply permutation: y = Pb
        let mut y = vec![0.0; n];
        for i in 0..n {
            y[i] = b[self.permutation[i]];
        }
        
        // Forward substitution: Ly = Pb (L has 1s on diagonal)
        for i in 0..n {
            for j in 0..i {
                y[i] -= self.lu.get(i, j) * y[j];
            }
        }
        
        // Backward substitution: Ux = y
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = y[i];
            for j in (i + 1)..n {
                sum -= self.lu.get(i, j) * x[j];
            }
            let diag = self.lu.get(i, i);
            if diag.abs() < 1e-12 {
                return Err(LpError::SingularBasis);
            }
            x[i] = sum / diag;
        }
        
        Ok(x)
    }

    /// Solve transpose system: A^T x = b
    ///
    /// Uses the existing LU decomposition: (LU)^T x = b => U^T L^T x = b
    pub fn solve_transpose(&self, b: &[f64]) -> Result<Vec<f64>, LpError> {
        let n = self.lu.rows;
        
        if b.len() != n {
            return Err(LpError::NumericalInstability);
        }
        
        // Solve U^T y = b (forward substitution since U^T is lower triangular)
        let mut y = vec![0.0; n];
        for i in 0..n {
            let mut sum = b[i];
            for j in 0..i {
                sum -= self.lu.get(j, i) * y[j];  // Note: transposed access
            }
            let diag = self.lu.get(i, i);
            if diag.abs() < 1e-12 {
                return Err(LpError::SingularBasis);
            }
            y[i] = sum / diag;
        }
        
        // Solve L^T x = y (backward substitution since L^T is upper triangular)
        // Note: L has 1s on diagonal
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = y[i];
            for j in (i + 1)..n {
                sum -= self.lu.get(j, i) * x[j];  // Note: transposed access
            }
            x[i] = sum;  // Diagonal is 1
        }
        
        // Apply inverse permutation: result = P^T x
        let mut result = vec![0.0; n];
        for i in 0..n {
            result[self.permutation[i]] = x[i];
        }
        
        Ok(result)
    }
    
    /// Solve multiple right-hand sides: AX = B
    pub fn solve_multiple(&self, b_matrix: &Matrix) -> Result<Matrix, LpError> {
        if b_matrix.rows != self.lu.rows {
            return Err(LpError::NumericalInstability);
        }
        
        let mut result = Matrix::zeros(b_matrix.rows, b_matrix.cols);
        
        for col in 0..b_matrix.cols {
            let b_col = b_matrix.col(col);
            let x = self.solve(&b_col)?;
            
            for row in 0..result.rows {
                result.set(row, col, x[row]);
            }
        }
        
        Ok(result)
    }
    
    /// Compute the determinant from LU decomposition
    ///
    /// det(A) = sign * product of diagonal elements of U
    pub fn determinant(&self) -> f64 {
        let mut det = self.sign as f64;
        for i in 0..self.lu.rows {
            det *= self.lu.get(i, i);
        }
        det
    }
    
    /// Extract L matrix (lower triangular with 1s on diagonal)
    pub fn extract_l(&self) -> Matrix {
        let n = self.lu.rows;
        let mut l = Matrix::identity(n);
        
        for i in 0..n {
            for j in 0..i {
                l.set(i, j, self.lu.get(i, j));
            }
        }
        l
    }
    
    /// Extract U matrix (upper triangular)
    pub fn extract_u(&self) -> Matrix {
        let n = self.lu.rows;
        let mut u = Matrix::zeros(n, n);
        
        for i in 0..n {
            for j in i..n {
                u.set(i, j, self.lu.get(i, j));
            }
        }
        u
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lu_decompose_simple() {
        // Simple 2x2 matrix
        let a = Matrix::from_rows(vec![
            vec![2.0, 1.0],
            vec![4.0, 3.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        
        // Verify we can solve a system
        let b = vec![5.0, 13.0]; 
        let x = lu.solve(&b).unwrap();
        
        // Check solution by verifying Ax = b
        let ax = a.mul_vec(&x);
        assert!((ax[0] - b[0]).abs() < 1e-10);
        assert!((ax[1] - b[1]).abs() < 1e-10);
    }
    
    #[test]
    fn test_lu_decompose_3x3() {
        let a = Matrix::from_rows(vec![
            vec![2.0, -1.0, 0.0],
            vec![-1.0, 2.0, -1.0],
            vec![0.0, -1.0, 2.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        
        // Solve Ax = b where b = [1, 0, 1]
        let b = vec![1.0, 0.0, 1.0];
        let x = lu.solve(&b).unwrap();
        
        // Verify: Ax should equal b
        let ax = a.mul_vec(&x);
        for i in 0..3 {
            assert!((ax[i] - b[i]).abs() < 1e-10);
        }
    }
    
    #[test]
    fn test_lu_identity() {
        let identity = Matrix::identity(3);
        let lu = LuDecomposition::decompose(&identity, 1e-10).unwrap();
        
        let b = vec![1.0, 2.0, 3.0];
        let x = lu.solve(&b).unwrap();
        
        // Identity matrix: x should equal b
        assert_eq!(x, b);
    }
    
    #[test]
    fn test_lu_singular_matrix() {
        // Singular matrix (second row is 2x first row)
        let a = Matrix::from_rows(vec![
            vec![1.0, 2.0],
            vec![2.0, 4.0],
        ]);
        
        let result = LuDecomposition::decompose(&a, 1e-10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LpError::SingularBasis);
    }
    
    #[test]
    fn test_lu_ill_conditioned_identical_rows() {
        // Ill-conditioned matrix with two identical rows
        // This represents a linearly dependent system (rank < n)
        let a = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],  // Identical to first row
            vec![4.0, 5.0, 6.0],
        ]);
        
        // Should fail as singular (rank-deficient)
        let result = LuDecomposition::decompose(&a, 1e-10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LpError::SingularBasis);
    }
    
    #[test]
    fn test_lu_with_pivoting() {
        // Matrix that requires pivoting for stability
        let a = Matrix::from_rows(vec![
            vec![0.0001, 1.0],
            vec![1.0, 1.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        
        let b = vec![1.0, 2.0];
        let x = lu.solve(&b).unwrap();
        
        // Verify solution
        let ax = a.mul_vec(&x);
        for i in 0..2 {
            assert!((ax[i] - b[i]).abs() < 1e-8);
        }
    }
    
    #[test]
    fn test_lu_determinant() {
        let a = Matrix::from_rows(vec![
            vec![2.0, 1.0],
            vec![4.0, 3.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        let det = lu.determinant();
        
        // det([[2,1],[4,3]]) = 2*3 - 1*4 = 2
        assert!((det - 2.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_lu_extract_l_u() {
        let a = Matrix::from_rows(vec![
            vec![4.0, 3.0],
            vec![6.0, 3.0],
        ]);
        
        let lu_decomp = LuDecomposition::decompose(&a, 1e-10).unwrap();
        let l = lu_decomp.extract_l();
        let u = lu_decomp.extract_u();
        
        // L should be lower triangular with 1s on diagonal
        assert_eq!(l.get(0, 0), 1.0);
        assert_eq!(l.get(1, 1), 1.0);
        assert_eq!(l.get(0, 1), 0.0); // Upper triangle is zero
        
        // U should be upper triangular
        assert_eq!(u.get(1, 0), 0.0); // Lower triangle is zero
        
        // PA = LU (where P is permutation)
        let lu_product = l.mul_matrix(&u);
        
        // Apply permutation to original matrix
        let mut pa = Matrix::zeros(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                pa.set(i, j, a.get(lu_decomp.permutation[i], j));
            }
        }
        
        // Check PA ≈ LU
        for i in 0..a.rows {
            for j in 0..a.cols {
                assert!((pa.get(i, j) - lu_product.get(i, j)).abs() < 1e-10);
            }
        }
    }
    
    #[test]
    fn test_lu_solve_multiple() {
        let a = Matrix::from_rows(vec![
            vec![2.0, 1.0],
            vec![4.0, 3.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        
        // Solve for multiple right-hand sides
        let b = Matrix::from_rows(vec![
            vec![5.0, 3.0],
            vec![13.0, 7.0],
        ]);
        
        let x = lu.solve_multiple(&b).unwrap();
        
        // Verify AX = B
        let ax = a.mul_matrix(&x);
        for i in 0..2 {
            for j in 0..2 {
                assert!((ax.get(i, j) - b.get(i, j)).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_solve_transpose() {
        // Test solving A^T x = b
        let a = Matrix::from_rows(vec![
            vec![2.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        
        // Solve A^T x = b where b = [1, 2, 3]
        let b = vec![1.0, 2.0, 3.0];
        let x = lu.solve_transpose(&b).unwrap();
        
        // Verify A^T x = b by computing the product
        let a_transpose = a.transpose();
        let result = a_transpose.mul_vec(&x);
        
        for i in 0..3 {
            assert!(
                (result[i] - b[i]).abs() < 1e-10,
                "A^T x = b verification failed at index {}: {} != {}",
                i,
                result[i],
                b[i]
            );
        }
    }

    #[test]
    fn test_lu_solve_transpose_simple() {
        // Simple 2x2 test
        // A = [[2, 1], [4, 3]]
        // A^T = [[2, 4], [1, 3]]
        let a = Matrix::from_rows(vec![
            vec![2.0, 1.0],
            vec![4.0, 3.0],
        ]);
        
        let lu = LuDecomposition::decompose(&a, 1e-10).unwrap();
        
        // Solve A^T x = [10, 7] where A^T = [[2, 4], [1, 3]]
        // Expected solution: x = [1, 2] because [[2,4],[1,3]] * [1,2] = [10, 7]
        let b2 = vec![10.0, 7.0];
        let x2 = lu.solve_transpose(&b2).unwrap();
        
        assert!((x2[0] - 1.0).abs() < 1e-10, "x[0] should be 1.0, got {}", x2[0]);
        assert!((x2[1] - 2.0).abs() < 1e-10, "x[1] should be 2.0, got {}", x2[1]);
    }
}
