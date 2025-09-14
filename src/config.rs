//! Configuration system for the CSP solver
//!
//! This module provides the `SolverConfig` struct for configuring solver behavior,
//! including precision settings, resource limits, and other solver parameters.

use crate::domain::float_interval::DEFAULT_FLOAT_PRECISION_DIGITS;

/// Configuration for the CSP solver
///
/// This struct contains all configurable parameters for the solver.
/// Use `SolverConfig::default()` for sensible defaults, or create a custom
/// configuration using the builder methods.
///
/// # Examples
///
/// ```rust
/// use cspsolver::prelude::*;
/// 
/// // Use default configuration
/// let config = SolverConfig::default();
/// let mut m = Model::with_config(config);
/// 
/// // Custom configuration
/// let config = SolverConfig::default()
///     .with_float_precision(6);
/// let mut m = Model::with_config(config);
/// ```
#[derive(Debug, Clone)]
pub struct SolverConfig {
    /// Precision for float variables (decimal places)
    pub float_precision_digits: i32,
    
    // Future configuration options (placeholders for next steps)
    /// Maximum time to spend solving (in seconds)
    /// None means no timeout
    pub timeout_seconds: Option<u64>,
    
    /// Maximum memory usage (in MB)
    /// None means no memory limit
    pub max_memory_mb: Option<u64>,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            float_precision_digits: DEFAULT_FLOAT_PRECISION_DIGITS,
            timeout_seconds: None,
            max_memory_mb: None,
        }
    }
}

impl SolverConfig {
    /// Create a new configuration with default values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cspsolver::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new();
    /// assert_eq!(config.float_precision_digits, 6);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the float precision (number of decimal places)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cspsolver::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new().with_float_precision(4);
    /// assert_eq!(config.float_precision_digits, 4);
    /// ```
    pub fn with_float_precision(mut self, precision_digits: i32) -> Self {
        self.float_precision_digits = precision_digits;
        self
    }
    
    /// Set the timeout in seconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cspsolver::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new().with_timeout_seconds(30);
    /// assert_eq!(config.timeout_seconds, Some(30));
    /// ```
    pub fn with_timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }
    
    /// Remove the timeout (allow unlimited solving time)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cspsolver::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new()
    ///     .with_timeout_seconds(30)
    ///     .without_timeout();
    /// assert_eq!(config.timeout_seconds, None);
    /// ```
    pub fn without_timeout(mut self) -> Self {
        self.timeout_seconds = None;
        self
    }
    
    /// Set the maximum memory usage in MB
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cspsolver::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new().with_max_memory_mb(1024);
    /// assert_eq!(config.max_memory_mb, Some(1024));
    /// ```
    pub fn with_max_memory_mb(mut self, mb: u64) -> Self {
        self.max_memory_mb = Some(mb);
        self
    }
    
    /// Remove the memory limit (allow unlimited memory usage)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cspsolver::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new()
    ///     .with_max_memory_mb(1024)
    ///     .without_memory_limit();
    /// assert_eq!(config.max_memory_mb, None);
    /// ```
    pub fn without_memory_limit(mut self) -> Self {
        self.max_memory_mb = None;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = SolverConfig::default();
        assert_eq!(config.float_precision_digits, DEFAULT_FLOAT_PRECISION_DIGITS);
        assert_eq!(config.timeout_seconds, None);
        assert_eq!(config.max_memory_mb, None);
    }
    
    #[test]
    fn test_builder_pattern() {
        let config = SolverConfig::new()
            .with_float_precision(4)
            .with_timeout_seconds(60)
            .with_max_memory_mb(512);
            
        assert_eq!(config.float_precision_digits, 4);
        assert_eq!(config.timeout_seconds, Some(60));
        assert_eq!(config.max_memory_mb, Some(512));
    }
    
    #[test]
    fn test_without_methods() {
        let config = SolverConfig::new()
            .with_timeout_seconds(30)
            .with_max_memory_mb(256)
            .without_timeout()
            .without_memory_limit();
            
        assert_eq!(config.timeout_seconds, None);
        assert_eq!(config.max_memory_mb, None);
    }
}