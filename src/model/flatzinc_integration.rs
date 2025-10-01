//! FlatZinc integration methods for Model

use crate::prelude::Model;
use crate::flatzinc::{parse_and_map, FlatZincResult};
use std::fs;
use std::path::Path;

impl Model {
    /// Import a FlatZinc file into this model.
    ///
    /// This allows you to configure the model (memory limits, timeout, etc.) before
    /// importing the FlatZinc problem.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the `.fzn` file
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or a `FlatZincError` if parsing or mapping fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use selen::prelude::*;
    ///
    /// // Configure model first
    /// let mut model = Model::default()
    ///     .with_timeout_seconds(30);
    ///
    /// model.from_flatzinc_file("problem.fzn")?;
    /// let solution = model.solve()?;
    /// ```
    pub fn from_flatzinc_file<P: AsRef<Path>>(&mut self, path: P) -> FlatZincResult<()> {
        let content = fs::read_to_string(path)
            .map_err(|e| crate::flatzinc::FlatZincError::IoError(e.to_string()))?;
        self.from_flatzinc_str(&content)
    }

    /// Import FlatZinc source code into this model.
    ///
    /// This allows you to configure the model (memory limits, timeout, etc.) before
    /// importing the FlatZinc problem.
    ///
    /// # Arguments
    ///
    /// * `content` - FlatZinc source code as a string
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or a `FlatZincError` if parsing or mapping fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use selen::prelude::*;
    ///
    /// let fzn = r#"
    ///     var 1..10: x;
    ///     var 1..10: y;
    ///     constraint int_eq(x, y);
    ///     solve satisfy;
    /// "#;
    ///
    /// let mut model = Model::default();
    /// model.from_flatzinc_str(fzn)?;
    /// let solution = model.solve()?;
    /// ```
    pub fn from_flatzinc_str(&mut self, content: &str) -> FlatZincResult<()> {
        parse_and_map(content, self)
    }
}
