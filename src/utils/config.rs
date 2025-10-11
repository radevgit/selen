//! Configuration system for the CSP solver
//!
//! This module provides the `SolverConfig` struct for configuring solver behavior,
//! including precision settings, resource limits, and other solver parameters.

use crate::variables::domain::float_interval::DEFAULT_FLOAT_PRECISION_DIGITS;

/// Configuration for the CSP solver
///
/// This struct contains all configurable parameters for the solver.
/// Use `SolverConfig::default()` for sensible defaults with automatic resource limits,
/// or create a custom configuration using the builder methods.
///
/// # Default Resource Limits (v0.6.0+)
///
/// All models now have automatic safety limits to prevent system memory exhaustion:
/// - **Memory limit**: 2GB (prevents crashes during variable creation)
/// - **Timeout**: 60000 milliseconds (60 seconds, prevents infinite solver runs)
/// - **Memory tracking**: Real-time during model building
///
/// # Examples
///
/// ```rust
/// use selen::prelude::*;
/// 
/// // Use default configuration (2GB memory, 60s timeout)
/// let config = SolverConfig::default();
/// let mut m = Model::with_config(config);
/// 
/// // Custom limits for production environments
/// let config = SolverConfig::default()
///     .with_float_precision(8)
///     .with_timeout_ms(120000)        // 120000ms = 2 minute timeout
///     .with_max_memory_mb(1024);      // 1GB memory limit
/// let mut m = Model::with_config(config);
/// 
/// // Remove all limits (use with caution!)
/// let config = SolverConfig::unlimited();
/// let mut m = Model::with_config(config);
/// ```
#[derive(Debug, Clone)]
pub struct SolverConfig {
    /// Precision for float variables (decimal places)
    /// Default: 6 decimal places
    pub float_precision_digits: i32,
    
    /// Maximum time to spend solving (in milliseconds)
    /// Default: Some(60000) - 60 second (1 minute) timeout
    /// None means no timeout (⚠️ use with caution)
    pub timeout_ms: Option<u64>,
    
    /// Maximum memory usage (in MB) during model building and solving
    /// Default: Some(2048) - 2GB memory limit
    /// None means no memory limit (⚠️ use with caution)
    pub max_memory_mb: Option<u64>,
    
    /// Expansion factor for unbounded variable inference
    /// When a variable is declared with unbounded/infinite bounds, this factor
    /// determines how much to expand beyond existing bounded variables.
    /// 
    /// Default: 1000 (expands context by 1000x)
    /// 
    /// **Examples**:
    /// - Factor 1000, context [0, 100] → infer [-100,000, 100,100]
    /// - Factor 300, context [0, 100] → infer [-30,000, 30,100]
    /// - Factor 10,000, context [0, 100] → may hit 1M domain limit for integers
    /// 
    /// **Rationale**:
    /// - Too small (< 100): May not provide enough exploration space
    /// - Too large (> 10,000): Often exceeds 1M domain limit for integers
    /// - 1000 is empirically good for most CSP/optimization problems
    /// - Advanced users can tune based on problem domain
    pub unbounded_inference_factor: u32,
    
    /// Enable LP (Linear Programming) solver for linear constraints
    /// 
    /// When enabled, the solver will automatically extract float linear constraints
    /// (float_lin_eq, float_lin_le) and solve them using the LP solver to tighten
    /// variable bounds. This can significantly improve performance on problems with
    /// many linear constraints.
    /// 
    /// Default: true (LP integration is always enabled by default for optimal performance)
    /// 
    /// **When to enable (default)**:
    /// - Models with ≥3 float linear constraints
    /// - Large variable domains (>1000 values)
    /// - Problems where interval propagation is slow
    /// 
    /// **When to disable (opt-out)**:
    /// - Models with few (<3) linear constraints
    /// - Small, discrete problems (LP overhead not worth it)
    /// - When you want pure CSP solving behavior
    /// 
    /// **Performance impact**:
    /// - Small problems: Slight overhead (~1-5ms)
    /// - Medium problems (10-50 vars): 2-10x faster
    /// - Large problems (50+ vars): 5-100x faster
    pub prefer_lp_solver: bool,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            float_precision_digits: DEFAULT_FLOAT_PRECISION_DIGITS,
            timeout_ms: Some(60000),     // Default 60000ms = 1 minute timeout
            max_memory_mb: Some(2048),   // Default 2GB memory limit
            unbounded_inference_factor: 1000, // Default 1000x expansion
            prefer_lp_solver: true,      // LP integration is always enabled by default
        }
    }
}

impl SolverConfig {
    /// Create a new configuration with default values
    ///
    /// Equivalent to `SolverConfig::default()` with automatic resource limits:
    /// - Memory limit: 2GB
    /// - Timeout: 60000 milliseconds (60 seconds)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new();
    /// assert_eq!(config.float_precision_digits, 6);
    /// assert_eq!(config.max_memory_mb, Some(2048));
    /// assert_eq!(config.timeout_ms, Some(60000));
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the float precision (number of decimal places)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new().with_float_precision(4);
    /// assert_eq!(config.float_precision_digits, 4);
    /// ```
    pub fn with_float_precision(mut self, precision_digits: i32) -> Self {
        self.float_precision_digits = precision_digits;
        self
    }
    
    /// Set the timeout in milliseconds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new().with_timeout_ms(30000);  // 30 seconds
    /// assert_eq!(config.timeout_ms, Some(30000));
    /// ```
    pub fn with_timeout_ms(mut self, milliseconds: u64) -> Self {
        self.timeout_ms = Some(milliseconds);
        self
    }
    
    /// Remove the timeout (allow unlimited solving time)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new()
    ///     .with_timeout_ms(30000)
    ///     .without_timeout();
    /// assert_eq!(config.timeout_ms, None);
    /// ```
    pub fn without_timeout(mut self) -> Self {
        self.timeout_ms = None;
        self
    }
    
    /// Set the maximum memory usage in MB
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
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
    /// use selen::prelude::config::SolverConfig;
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
    
    /// Set the expansion factor for unbounded variable inference
    ///
    /// When variables are declared with unbounded/infinite bounds (i32::MIN/MAX,
    /// f64::INFINITY), Selen infers reasonable bounds by expanding existing
    /// bounded variables by this factor.
    ///
    /// **Default**: 1000 (expand context by 1000x)
    ///
    /// **Guidelines**:
    /// - **100-500**: Conservative, for tightly constrained problems
    /// - **1000**: Good default for most problems (logarithmic middle ground)
    /// - **5000-10000**: Aggressive, for problems needing wide exploration
    ///
    /// **Note**: Larger factors may cause integer domains to exceed the 1M limit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// // Conservative inference (300x expansion)
    /// let config = SolverConfig::new().with_unbounded_inference_factor(300);
    /// assert_eq!(config.unbounded_inference_factor, 300);
    /// 
    /// // Aggressive inference (5000x expansion)
    /// let config = SolverConfig::new().with_unbounded_inference_factor(5000);
    /// assert_eq!(config.unbounded_inference_factor, 5000);
    /// ```
    pub fn with_unbounded_inference_factor(mut self, factor: u32) -> Self {
        self.unbounded_inference_factor = factor;
        self
    }
    
    /// Enable LP solver for linear constraints
    ///
    /// When enabled, the solver will automatically extract float linear constraints
    /// and solve them using the LP solver to tighten variable bounds. This can
    /// provide significant performance improvements on problems with many linear
    /// constraints and large domains.
    ///
    /// # Performance Guidelines
    ///
    /// **Enable for**:
    /// - Models with ≥3 float linear constraints
    /// - Large variable domains (>1000 values)
    /// - Problems where interval propagation is slow
    ///
    /// **Disable for**:
    /// - Models with few (<3) linear constraints
    /// - Small, discrete problems (LP overhead not beneficial)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// // Enable LP solver for linear constraint-heavy problems
    /// let config = SolverConfig::new().with_lp_solver();
    /// assert_eq!(config.prefer_lp_solver, true);
    /// ```
    pub fn with_lp_solver(mut self) -> Self {
        self.prefer_lp_solver = true;
        self
    }
    
    /// Disable LP solver (use pure CSP propagation only)
    ///
    /// Explicitly disables LP solver integration, forcing the solver to use
    /// only interval propagation for linear constraints. This is the default
    /// behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// let config = SolverConfig::new()
    ///     .with_lp_solver()
    ///     .without_lp_solver();
    /// assert_eq!(config.prefer_lp_solver, false);
    /// ```
    pub fn without_lp_solver(mut self) -> Self {
        self.prefer_lp_solver = false;
        self
    }
    
    /// Create a configuration with no limits (unlimited time and memory)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use selen::prelude::config::SolverConfig;
    /// 
    /// let config = SolverConfig::unlimited();
    /// assert_eq!(config.timeout_ms, None);
    /// assert_eq!(config.max_memory_mb, None);
    /// ```
    pub fn unlimited() -> Self {
        Self {
            float_precision_digits: DEFAULT_FLOAT_PRECISION_DIGITS,
            timeout_ms: None,
            max_memory_mb: None,
            unbounded_inference_factor: 1000, // Default 1000x expansion
            prefer_lp_solver: false,          // LP integration is opt-in
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = SolverConfig::default();
        assert_eq!(config.float_precision_digits, DEFAULT_FLOAT_PRECISION_DIGITS);
        assert_eq!(config.timeout_ms, Some(60000));        // Default 60000ms = 1 minute
        assert_eq!(config.max_memory_mb, Some(2048));      // Default 2GB
    }
    
    #[test]
    fn test_unlimited_config() {
        let config = SolverConfig::unlimited();
        assert_eq!(config.float_precision_digits, DEFAULT_FLOAT_PRECISION_DIGITS);
        assert_eq!(config.timeout_ms, None);
        assert_eq!(config.max_memory_mb, None);
    }
    
    #[test]
    fn test_builder_pattern() {
        let config = SolverConfig::new()
            .with_float_precision(4)
            .with_timeout_ms(60000)  // 60 seconds
            .with_max_memory_mb(512);
            
        assert_eq!(config.float_precision_digits, 4);
        assert_eq!(config.timeout_ms, Some(60000));
        assert_eq!(config.max_memory_mb, Some(512));
    }
    
    #[test]
    fn test_without_methods() {
        let config = SolverConfig::new()
            .with_timeout_ms(30000)  // 30 seconds
            .with_max_memory_mb(256)
            .without_timeout()
            .without_memory_limit();
            
        assert_eq!(config.timeout_ms, None);
        assert_eq!(config.max_memory_mb, None);
    }
    
    #[test]
    fn test_lp_solver_flag() {
        // Default is enabled
        let config = SolverConfig::new();
        assert_eq!(config.prefer_lp_solver, true);
        
        // Can disable
        let config = SolverConfig::new().without_lp_solver();
        assert_eq!(config.prefer_lp_solver, false);
        
        // Can re-enable after disabling
        let config = SolverConfig::new()
            .without_lp_solver()
            .with_lp_solver();
        assert_eq!(config.prefer_lp_solver, true);
    }
}
