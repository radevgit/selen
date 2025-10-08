//! Dense matrix operations for LP solver
//!
//! Provides basic linear algebra operations needed for the Simplex method.
//! Uses row-major storage format for cache efficiency.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global memory tracker for LP solver matrices
static LP_MEMORY_BYTES: AtomicU64 = AtomicU64::new(0);

/// Get current memory usage of LP solver matrices in bytes
pub fn get_lp_memory_bytes() -> u64 {
    LP_MEMORY_BYTES.load(Ordering::Relaxed)
}

/// Get current memory usage of LP solver matrices in MB
pub fn get_lp_memory_mb() -> f64 {
    get_lp_memory_bytes() as f64 / (1024.0 * 1024.0)
}

/// Reset memory tracker (useful for testing)
pub fn reset_lp_memory() {
    LP_MEMORY_BYTES.store(0, Ordering::Relaxed);
}

/// Dense matrix stored in row-major format
#[derive(Debug, PartialEq)]
pub struct Matrix {
    /// Number of rows
    pub rows: usize,
    
    /// Number of columns
    pub cols: usize,
    
    /// Data in row-major order: data[i*cols + j] = element at (i, j)
    pub data: Vec<f64>,
}

impl Matrix {
    /// Create a new matrix with given dimensions, initialized to zero
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let size_bytes = (rows * cols * std::mem::size_of::<f64>()) as u64;
        LP_MEMORY_BYTES.fetch_add(size_bytes, Ordering::Relaxed);
        
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }
    
    /// Create a new matrix with given dimensions, initialized to given value
    pub fn filled(rows: usize, cols: usize, value: f64) -> Self {
        let size_bytes = (rows * cols * std::mem::size_of::<f64>()) as u64;
        LP_MEMORY_BYTES.fetch_add(size_bytes, Ordering::Relaxed);
        
        Self {
            rows,
            cols,
            data: vec![value; rows * cols],
        }
    }
    
    /// Create an identity matrix of given size
    pub fn identity(size: usize) -> Self {
        let size_bytes = (size * size * std::mem::size_of::<f64>()) as u64;
        LP_MEMORY_BYTES.fetch_add(size_bytes, Ordering::Relaxed);
        
        let mut data = vec![0.0; size * size];
        for i in 0..size {
            data[i * size + i] = 1.0;
        }
        Self {
            rows: size,
            cols: size,
            data,
        }
    }
    
    /// Create a matrix from row vectors
    pub fn from_rows(rows: Vec<Vec<f64>>) -> Self {
        if rows.is_empty() {
            return Self {
                rows: 0,
                cols: 0,
                data: vec![],
            };
        }
        
        let n_rows = rows.len();
        let n_cols = rows[0].len();
        let mut data = Vec::with_capacity(n_rows * n_cols);
        
        for row in rows {
            assert_eq!(row.len(), n_cols, "All rows must have same length");
            data.extend(row);
        }
        
        let size_bytes = (n_rows * n_cols * std::mem::size_of::<f64>()) as u64;
        LP_MEMORY_BYTES.fetch_add(size_bytes, Ordering::Relaxed);
        
        Self {
            rows: n_rows,
            cols: n_cols,
            data,
        }
    }
    
    /// Get element at (row, col)
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        debug_assert!(row < self.rows, "Row index out of bounds");
        debug_assert!(col < self.cols, "Column index out of bounds");
        self.data[row * self.cols + col]
    }
    
    /// Set element at (row, col)
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        debug_assert!(row < self.rows, "Row index out of bounds");
        debug_assert!(col < self.cols, "Column index out of bounds");
        self.data[row * self.cols + col] = value;
    }
    
    /// Get a row as a slice
    pub fn row(&self, row: usize) -> &[f64] {
        debug_assert!(row < self.rows, "Row index out of bounds");
        let start = row * self.cols;
        &self.data[start..start + self.cols]
    }
    
    /// Get a mutable row slice
    pub fn row_mut(&mut self, row: usize) -> &mut [f64] {
        debug_assert!(row < self.rows, "Row index out of bounds");
        let start = row * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
    
    /// Extract a column as a new vector
    pub fn col(&self, col: usize) -> Vec<f64> {
        debug_assert!(col < self.cols, "Column index out of bounds");
        (0..self.rows)
            .map(|row| self.get(row, col))
            .collect()
    }
    
    /// Transpose the matrix
    pub fn transpose(&self) -> Self {
        let mut result = Self::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }
    
    /// Matrix-vector multiplication: self * vec
    pub fn mul_vec(&self, vec: &[f64]) -> Vec<f64> {
        assert_eq!(
            vec.len(),
            self.cols,
            "Vector length must equal number of columns"
        );
        
        let mut result = vec![0.0; self.rows];
        for i in 0..self.rows {
            let mut sum = 0.0;
            for j in 0..self.cols {
                sum += self.get(i, j) * vec[j];
            }
            result[i] = sum;
        }
        result
    }
    
    /// Matrix-matrix multiplication: self * other
    pub fn mul_matrix(&self, other: &Matrix) -> Self {
        assert_eq!(
            self.cols,
            other.rows,
            "Number of columns in first matrix must equal rows in second"
        );
        
        let mut result = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }
    
    /// Swap two rows
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 == row2 {
            return;
        }
        debug_assert!(row1 < self.rows && row2 < self.rows, "Row indices out of bounds");
        
        for j in 0..self.cols {
            let idx1 = row1 * self.cols + j;
            let idx2 = row2 * self.cols + j;
            self.data.swap(idx1, idx2);
        }
    }
    
    /// Scale a row by a scalar: row[i] *= scalar
    pub fn scale_row(&mut self, row: usize, scalar: f64) {
        debug_assert!(row < self.rows, "Row index out of bounds");
        let start = row * self.cols;
        for i in start..start + self.cols {
            self.data[i] *= scalar;
        }
    }
    
    /// Add a scaled row to another: row[target] += scalar * row[source]
    pub fn add_scaled_row(&mut self, target: usize, source: usize, scalar: f64) {
        debug_assert!(target < self.rows && source < self.rows, "Row indices out of bounds");
        
        for j in 0..self.cols {
            let target_idx = target * self.cols + j;
            let source_idx = source * self.cols + j;
            self.data[target_idx] += scalar * self.data[source_idx];
        }
    }
    
    /// Extract a submatrix by selecting specific rows and columns
    pub fn submatrix(&self, row_indices: &[usize], col_indices: &[usize]) -> Self {
        let mut result = Self::zeros(row_indices.len(), col_indices.len());
        
        for (i, &row_idx) in row_indices.iter().enumerate() {
            for (j, &col_idx) in col_indices.iter().enumerate() {
                result.set(i, j, self.get(row_idx, col_idx));
            }
        }
        result
    }
    
    /// Check if matrix contains any NaN or Inf values
    pub fn is_finite(&self) -> bool {
        self.data.iter().all(|x| x.is_finite())
    }
    
    /// Compute the Frobenius norm (L2 norm of all elements)
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    
    /// Get memory usage of this matrix in bytes
    pub fn memory_bytes(&self) -> u64 {
        (self.data.len() * std::mem::size_of::<f64>()) as u64
    }
}

// Manual Clone implementation to track memory on copy
impl Clone for Matrix {
    fn clone(&self) -> Self {
        let size_bytes = (self.data.len() * std::mem::size_of::<f64>()) as u64;
        LP_MEMORY_BYTES.fetch_add(size_bytes, Ordering::Relaxed);
        
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        }
    }
}

// Track deallocation
impl Drop for Matrix {
    fn drop(&mut self) {
        let size_bytes = (self.data.len() * std::mem::size_of::<f64>()) as u64;
        LP_MEMORY_BYTES.fetch_sub(size_bytes, Ordering::Relaxed);
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{} [", self.rows, self.cols)?;
        for i in 0..self.rows {
            write!(f, "  [")?;
            for j in 0..self.cols {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.4}", self.get(i, j))?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = Matrix::zeros(2, 3);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data.len(), 6);
        assert!(m.data.iter().all(|&x| x == 0.0));
        
        let m = Matrix::filled(2, 2, 5.0);
        assert!(m.data.iter().all(|&x| x == 5.0));
        
        let m = Matrix::identity(3);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(0, 1), 0.0);
        assert_eq!(m.get(1, 0), 0.0);
    }
    
    #[test]
    fn test_from_rows() {
        let m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 2), 3.0);
        assert_eq!(m.get(1, 0), 4.0);
        assert_eq!(m.get(1, 2), 6.0);
    }
    
    #[test]
    fn test_get_set() {
        let mut m = Matrix::zeros(2, 2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 2.0);
        m.set(1, 0, 3.0);
        m.set(1, 1, 4.0);
        
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(1, 1), 4.0);
    }
    
    #[test]
    fn test_row_col_access() {
        let m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);
        
        let row0 = m.row(0);
        assert_eq!(row0, &[1.0, 2.0, 3.0]);
        
        let row1 = m.row(1);
        assert_eq!(row1, &[4.0, 5.0, 6.0]);
        
        let col0 = m.col(0);
        assert_eq!(col0, vec![1.0, 4.0]);
        
        let col2 = m.col(2);
        assert_eq!(col2, vec![3.0, 6.0]);
    }
    
    #[test]
    fn test_memory_tracking() {
        // Reset memory counter
        reset_lp_memory();
        assert_eq!(get_lp_memory_bytes(), 0);
        
        // Create a matrix and check memory increased
        let m1 = Matrix::zeros(10, 10);
        let expected_bytes = (100 * std::mem::size_of::<f64>()) as u64;
        assert_eq!(get_lp_memory_bytes(), expected_bytes);
        assert!((get_lp_memory_mb() - (expected_bytes as f64 / 1024.0 / 1024.0)).abs() < 1e-9);
        
        // Clone and check memory doubled
        let m2 = m1.clone();
        assert_eq!(get_lp_memory_bytes(), 2 * expected_bytes);
        
        // Drop one and check memory halved
        drop(m1);
        assert_eq!(get_lp_memory_bytes(), expected_bytes);
        
        // Drop the other and check memory is zero
        drop(m2);
        assert_eq!(get_lp_memory_bytes(), 0);
    }

    #[test]
    fn test_transpose() {
        let m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);
        
        let mt = m.transpose();
        assert_eq!(mt.rows, 3);
        assert_eq!(mt.cols, 2);
        assert_eq!(mt.get(0, 0), 1.0);
        assert_eq!(mt.get(0, 1), 4.0);
        assert_eq!(mt.get(1, 0), 2.0);
        assert_eq!(mt.get(1, 1), 5.0);
        assert_eq!(mt.get(2, 0), 3.0);
        assert_eq!(mt.get(2, 1), 6.0);
    }
    
    #[test]
    fn test_mul_vec() {
        let m = Matrix::from_rows(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ]);
        
        let v = vec![5.0, 6.0];
        let result = m.mul_vec(&v);
        
        // [1 2] * [5]   [1*5 + 2*6]   [17]
        // [3 4]   [6] = [3*5 + 4*6] = [39]
        assert_eq!(result, vec![17.0, 39.0]);
    }
    
    #[test]
    fn test_mul_matrix() {
        let m1 = Matrix::from_rows(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ]);
        
        let m2 = Matrix::from_rows(vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ]);
        
        let result = m1.mul_matrix(&m2);
        
        // [1 2] * [5 6]   [1*5+2*7  1*6+2*8]   [19 22]
        // [3 4]   [7 8] = [3*5+4*7  3*6+4*8] = [43 50]
        assert_eq!(result.get(0, 0), 19.0);
        assert_eq!(result.get(0, 1), 22.0);
        assert_eq!(result.get(1, 0), 43.0);
        assert_eq!(result.get(1, 1), 50.0);
    }
    
    #[test]
    fn test_swap_rows() {
        let mut m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);
        
        m.swap_rows(0, 1);
        assert_eq!(m.row(0), &[4.0, 5.0, 6.0]);
        assert_eq!(m.row(1), &[1.0, 2.0, 3.0]);
    }
    
    #[test]
    fn test_scale_row() {
        let mut m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);
        
        m.scale_row(0, 2.0);
        assert_eq!(m.row(0), &[2.0, 4.0, 6.0]);
        assert_eq!(m.row(1), &[4.0, 5.0, 6.0]); // Unchanged
    }
    
    #[test]
    fn test_add_scaled_row() {
        let mut m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);
        
        // row[1] += 2.0 * row[0]
        m.add_scaled_row(1, 0, 2.0);
        assert_eq!(m.row(0), &[1.0, 2.0, 3.0]); // Unchanged
        assert_eq!(m.row(1), &[6.0, 9.0, 12.0]); // [4+2*1, 5+2*2, 6+2*3]
    }
    
    #[test]
    fn test_submatrix() {
        let m = Matrix::from_rows(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);
        
        // Extract rows 0,2 and columns 1,2
        let sub = m.submatrix(&[0, 2], &[1, 2]);
        assert_eq!(sub.rows, 2);
        assert_eq!(sub.cols, 2);
        assert_eq!(sub.get(0, 0), 2.0);
        assert_eq!(sub.get(0, 1), 3.0);
        assert_eq!(sub.get(1, 0), 8.0);
        assert_eq!(sub.get(1, 1), 9.0);
    }
    
    #[test]
    fn test_is_finite() {
        let m = Matrix::from_rows(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ]);
        assert!(m.is_finite());
        
        let mut m_nan = m.clone();
        m_nan.set(0, 0, f64::NAN);
        assert!(!m_nan.is_finite());
        
        let mut m_inf = m.clone();
        m_inf.set(1, 1, f64::INFINITY);
        assert!(!m_inf.is_finite());
    }
    
    #[test]
    fn test_frobenius_norm() {
        let m = Matrix::from_rows(vec![
            vec![3.0, 4.0],
            vec![0.0, 0.0],
        ]);
        
        // Frobenius norm = sqrt(3^2 + 4^2 + 0 + 0) = sqrt(25) = 5.0
        assert!((m.frobenius_norm() - 5.0).abs() < 1e-10);
    }
}
