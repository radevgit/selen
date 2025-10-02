//! Global cardinality constraint mappers
//!
//! Maps FlatZinc global cardinality constraint to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::ModelExt;

impl<'a> MappingContext<'a> {
    /// Map global_cardinality: For each value[i], count occurrences in vars array
    /// FlatZinc signature: global_cardinality(vars, values, counts)
    /// 
    /// Where:
    /// - vars: array of variables to count in
    /// - values: array of values to count (must be constants)
    /// - counts: array of count variables (one per value)
    /// 
    /// Constraint: For each i, counts[i] = |{j : vars[j] = values[i]}|
    pub(in crate::flatzinc::mapper) fn map_global_cardinality(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 3 {
            return Err(FlatZincError::MapError {
                message: "global_cardinality requires 3 arguments (vars, values, counts)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Extract the variables array
        let vars = self.extract_var_array(&constraint.args[0])?;
        
        // Extract the values array (must be constants)
        let values = self.extract_int_array(&constraint.args[1])?;
        
        // Extract the counts array (variables or constants)
        let counts = self.extract_var_array(&constraint.args[2])?;
        
        // Verify arrays have compatible sizes
        if values.len() != counts.len() {
            return Err(FlatZincError::MapError {
                message: format!(
                    "global_cardinality: values array length ({}) must match counts array length ({})",
                    values.len(),
                    counts.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // For each value, create a count constraint
        for (&value, &count_var) in values.iter().zip(counts.iter()) {
            // Use Selen's count constraint: count(vars, value, count_var)
            // This constrains: count_var = |{j : vars[j] = value}|
            self.model.count(&vars, value, count_var);
        }
        
        Ok(())
    }
    
    /// Map global_cardinality_low_up_closed: Count with bounds on counts
    /// FlatZinc signature: global_cardinality_low_up_closed(vars, values, low, up)
    /// 
    /// Where:
    /// - vars: array of variables to count in
    /// - values: array of values to count (must be constants)
    /// - low: array of lower bounds for counts
    /// - up: array of upper bounds for counts
    /// 
    /// Constraint: For each i, low[i] <= |{j : vars[j] = values[i]}| <= up[i]
    pub(in crate::flatzinc::mapper) fn map_global_cardinality_low_up_closed(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 4 {
            return Err(FlatZincError::MapError {
                message: "global_cardinality_low_up_closed requires 4 arguments (vars, values, low, up)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Extract the variables array
        let vars = self.extract_var_array(&constraint.args[0])?;
        
        // Extract the values array (must be constants)
        let values = self.extract_int_array(&constraint.args[1])?;
        
        // Extract the low bounds array
        let low = self.extract_int_array(&constraint.args[2])?;
        
        // Extract the up bounds array
        let up = self.extract_int_array(&constraint.args[3])?;
        
        // Verify arrays have compatible sizes
        if values.len() != low.len() || values.len() != up.len() {
            return Err(FlatZincError::MapError {
                message: format!(
                    "global_cardinality_low_up_closed: arrays must have same length (values: {}, low: {}, up: {})",
                    values.len(),
                    low.len(),
                    up.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // For each value, create a count variable and constrain it with bounds
        for i in 0..values.len() {
            let value = values[i];
            let low_bound = low[i];
            let up_bound = up[i];
            
            // Create a variable to hold the count with appropriate bounds
            let count_var = self.model.int(low_bound, up_bound);
            
            // Use Selen's count constraint: count(vars, value, count_var)
            // This constrains: count_var = |{j : vars[j] = value}|
            // The count_var's domain already enforces low_bound <= count_var <= up_bound
            self.model.count(&vars, value, count_var);
        }
        
        Ok(())
    }
}
