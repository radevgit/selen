//! FlatZinc Output Formatter
//!
//! Formats solution results according to FlatZinc output specification.

use crate::variables::{Val, VarId};
use std::collections::HashMap;

/// Format a solution in FlatZinc output format
pub fn format_solution(
    solution: &HashMap<VarId, Val>,
    var_names: &HashMap<VarId, String>,
) -> String {
    let mut output = String::new();
    
    // Sort variables by name for consistent output
    let mut sorted_vars: Vec<_> = var_names.iter().collect();
    sorted_vars.sort_by_key(|(_, name)| name.as_str());
    
    for (var_id, name) in sorted_vars {
        if let Some(val) = solution.get(var_id) {
            let value_str = match val {
                Val::ValI(i) => i.to_string(),
                Val::ValF(f) => f.to_string(),
            };
            output.push_str(&format!("{} = {};\n", name, value_str));
        }
    }
    
    output.push_str("----------\n");
    output
}

/// Format "no solution" message
pub fn format_no_solution() -> String {
    "=====UNSATISFIABLE=====\n".to_string()
}

/// Format "unknown" message (solver could not determine satisfiability)
pub fn format_unknown() -> String {
    "=====UNKNOWN=====\n".to_string()
}
