# Trait-Based Custom Functions Design for Selen Constraint Solver

## Overview

This document details the design and implementation of a **trait-based custom function system** for the Selen constraint solver. This system will allow end users to extend the solver's capabilities with domain-specific logic, integration hooks, and specialized constraints through well-defined Rust traits.

## Design Decision: Trait-Based System

After evaluation of multiple approaches (callbacks, plugins, declarative configs), we have chosen the **trait-based system** for the following reasons:

- **Type Safety**: Full compile-time type checking and Rust safety guarantees
- **Performance**: Zero-cost abstractions with trait objects when needed
- **Composability**: Easy to combine and layer multiple custom functions
- **Testability**: Individual traits can be unit tested in isolation
- **Documentation**: Self-documenting through trait definitions and rustdoc
- **IDE Support**: Full autocomplete, refactoring, and error checking
- **Rust Ecosystem Fit**: Idiomatic Rust design that leverages the type system

## Core Trait Definitions

### 1. Custom Constraint Trait

```rust
/// Trait for implementing custom constraint logic
pub trait CustomConstraint: Send + Sync {
    /// Validate a complete solution against this constraint
    fn validate(&self, solution: &Solution) -> ConstraintResult;
    
    /// Validate a partial solution (for early pruning)
    fn validate_partial(&self, partial: &PartialSolution) -> ConstraintResult {
        // Default implementation - only check complete solutions
        ConstraintResult::Unknown
    }
    
    /// Get a human-readable description of this constraint
    fn description(&self) -> String;
    
    /// Get the priority/importance of this constraint (0.0 = low, 1.0 = critical)
    fn priority(&self) -> f64 { 0.5 }
    
    /// Whether this constraint can be violated with a penalty (soft constraint)
    fn is_soft(&self) -> bool { false }
    
    /// Calculate violation penalty for soft constraints
    fn violation_penalty(&self, solution: &Solution) -> f64 { 0.0 }
}

/// Result of constraint validation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstraintResult {
    /// Constraint is satisfied
    Satisfied,
    /// Constraint is violated (hard failure)
    Violated,
    /// Cannot determine yet (partial solution)
    Unknown,
    /// Soft constraint violated with given penalty
    SoftViolation(f64),
}
```

### 2. Custom Objective Function Trait

```rust
/// Trait for implementing custom optimization objectives
pub trait CustomObjective: Send + Sync {
    /// Evaluate the objective function for a complete solution
    fn evaluate(&self, solution: &Solution) -> f64;
    
    /// Whether this is a minimization (true) or maximization (false) objective
    fn is_minimizing(&self) -> bool { true }
    
    /// Get a description of this objective
    fn description(&self) -> String;
    
    /// Get the weight/importance of this objective in multi-objective optimization
    fn weight(&self) -> f64 { 1.0 }
    
    /// Estimate objective value for partial solutions (for heuristics)
    fn estimate_partial(&self, partial: &PartialSolution) -> Option<f64> {
        None // Default: cannot estimate
    }
    
    /// Get bounds for this objective (min, max) if known
    fn bounds(&self) -> Option<(f64, f64)> {
        None
    }
}
```

### 3. Custom Heuristic Trait

```rust
/// Trait for implementing custom search heuristics
pub trait CustomHeuristic: Send + Sync {
    /// Select the next variable to assign in the search
    fn select_variable(&self, partial: &PartialSolution, candidates: &[VarId]) -> Option<VarId>;
    
    /// Select the order of values to try for a variable
    fn order_values(&self, var: VarId, domain: &[i32], partial: &PartialSolution) -> Vec<i32>;
    
    /// Get a description of this heuristic
    fn description(&self) -> String;
    
    /// Priority of this heuristic (higher = more important)
    fn priority(&self) -> u32 { 100 }
}
```

### 4. Custom Propagator Trait

```rust
/// Trait for implementing custom constraint propagation
pub trait CustomPropagator: Send + Sync {
    /// Propagate constraints and prune domains
    fn propagate(&self, domains: &mut DomainMap) -> PropagationResult;
    
    /// Variables this propagator depends on
    fn watched_variables(&self) -> &[VarId];
    
    /// Called when a watched variable's domain changes
    fn on_domain_change(&mut self, var: VarId, old_domain: &Domain, new_domain: &Domain);
    
    /// Get a description of this propagator
    fn description(&self) -> String;
    
    /// Whether this propagator should run at every node (true) or only when triggered (false)
    fn is_eager(&self) -> bool { false }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropagationResult {
    /// No changes made
    NoChange,
    /// Some domains were pruned
    Changed,
    /// Inconsistency detected (no solution possible)
    Inconsistent,
}
```

### 5. Integration Hook Trait

```rust
/// Trait for implementing integration hooks and callbacks
pub trait IntegrationHook: Send + Sync {
    /// Called when the solver starts
    fn on_solve_start(&mut self, model: &Model) {}
    
    /// Called when a solution is found
    fn on_solution_found(&mut self, solution: &Solution) {}
    
    /// Called when the solver finishes (with or without solution)
    fn on_solve_complete(&mut self, result: &SolveResult) {}
    
    /// Called periodically during search (for progress updates)
    fn on_search_progress(&mut self, stats: &SearchStatistics) {}
    
    /// Called when search backtracks
    fn on_backtrack(&mut self, level: usize) {}
    
    /// Get a description of this hook
    fn description(&self) -> String;
}
```

## Usage Examples

### Example 1: Workload Balance Constraint

```rust
use selen::prelude::*;

/// Ensures workload is balanced across employees
struct WorkloadBalanceConstraint {
    max_variance: f64,
    employee_weights: HashMap<usize, f64>,
}

impl WorkloadBalanceConstraint {
    pub fn new(max_variance: f64) -> Self {
        Self {
            max_variance,
            employee_weights: HashMap::new(),
        }
    }
    
    pub fn with_weights(mut self, weights: HashMap<usize, f64>) -> Self {
        self.employee_weights = weights;
        self
    }
}

impl CustomConstraint for WorkloadBalanceConstraint {
    fn validate(&self, solution: &Solution) -> ConstraintResult {
        let workloads = calculate_employee_workloads(solution, &self.employee_weights);
        let variance = calculate_variance(&workloads);
        
        if variance <= self.max_variance {
            ConstraintResult::Satisfied
        } else {
            // Soft constraint - return penalty proportional to excess variance
            let penalty = (variance - self.max_variance) * 10.0;
            ConstraintResult::SoftViolation(penalty)
        }
    }
    
    fn description(&self) -> String {
        format!("Workload variance must not exceed {}", self.max_variance)
    }
    
    fn is_soft(&self) -> bool { true }
    
    fn violation_penalty(&self, solution: &Solution) -> f64 {
        let workloads = calculate_employee_workloads(solution, &self.employee_weights);
        let variance = calculate_variance(&workloads);
        (variance - self.max_variance).max(0.0) * 10.0
    }
}

// Helper functions
fn calculate_employee_workloads(solution: &Solution, weights: &HashMap<usize, f64>) -> Vec<f64> {
    // Implementation here
    todo!()
}

fn calculate_variance(values: &[f64]) -> f64 {
    // Implementation here
    todo!()
}
```

### Example 2: Minimize Commute Cost Objective

```rust
/// Objective to minimize total commute costs
struct MinimizeCommuteCost {
    employee_locations: HashMap<usize, (f64, f64)>, // (lat, lng)
    workplace_locations: HashMap<usize, (f64, f64)>,
    cost_per_mile: f64,
}

impl MinimizeCommuteCost {
    pub fn new(cost_per_mile: f64) -> Self {
        Self {
            employee_locations: HashMap::new(),
            workplace_locations: HashMap::new(),
            cost_per_mile,
        }
    }
    
    pub fn add_employee_location(mut self, emp_id: usize, lat: f64, lng: f64) -> Self {
        self.employee_locations.insert(emp_id, (lat, lng));
        self
    }
    
    pub fn add_workplace_location(mut self, workplace_id: usize, lat: f64, lng: f64) -> Self {
        self.workplace_locations.insert(workplace_id, (lat, lng));
        self
    }
}

impl CustomObjective for MinimizeCommuteCost {
    fn evaluate(&self, solution: &Solution) -> f64 {
        let mut total_cost = 0.0;
        
        for (emp_id, &emp_location) in &self.employee_locations {
            if let Some(workplace_id) = get_employee_assignment(solution, *emp_id) {
                if let Some(&workplace_location) = self.workplace_locations.get(&workplace_id) {
                    let distance = haversine_distance(emp_location, workplace_location);
                    total_cost += distance * self.cost_per_mile;
                }
            }
        }
        
        total_cost
    }
    
    fn description(&self) -> String {
        format!("Minimize total commute cost at ${:.2} per mile", self.cost_per_mile)
    }
    
    fn bounds(&self) -> Option<(f64, f64)> {
        // Minimum: everyone works from home (0 cost)
        // Maximum: everyone has worst-case commute
        Some((0.0, self.calculate_max_possible_cost()))
    }
}

impl MinimizeCommuteCost {
    fn calculate_max_possible_cost(&self) -> f64 {
        // Calculate theoretical maximum cost
        todo!()
    }
}

// Helper functions
fn get_employee_assignment(solution: &Solution, emp_id: usize) -> Option<usize> {
    // Extract workplace assignment from solution
    todo!()
}

fn haversine_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    // Calculate distance between two lat/lng points
    todo!()
}
```

### Example 3: Domain-Specific Heuristic

```rust
/// Healthcare-specific scheduling heuristic
struct HealthcareSchedulingHeuristic {
    nurse_experience_levels: HashMap<usize, ExperienceLevel>,
    shift_criticality: HashMap<usize, CriticalityLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ExperienceLevel { Novice, Intermediate, Senior, Expert }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CriticalityLevel { Low, Medium, High, Critical }

impl HealthcareSchedulingHeuristic {
    pub fn new() -> Self {
        Self {
            nurse_experience_levels: HashMap::new(),
            shift_criticality: HashMap::new(),
        }
    }
    
    pub fn add_nurse_experience(mut self, nurse_id: usize, level: ExperienceLevel) -> Self {
        self.nurse_experience_levels.insert(nurse_id, level);
        self
    }
    
    pub fn add_shift_criticality(mut self, shift_id: usize, level: CriticalityLevel) -> Self {
        self.shift_criticality.insert(shift_id, level);
        self
    }
}

impl CustomHeuristic for HealthcareSchedulingHeuristic {
    fn select_variable(&self, partial: &PartialSolution, candidates: &[VarId]) -> Option<VarId> {
        // Priority: assign critical shifts first
        candidates.iter()
            .filter_map(|&var| {
                let shift_id = extract_shift_id_from_var(var)?;
                let criticality = self.shift_criticality.get(&shift_id)?;
                Some((var, criticality))
            })
            .max_by_key(|(_, criticality)| *criticality)
            .map(|(var, _)| var)
    }
    
    fn order_values(&self, var: VarId, domain: &[i32], partial: &PartialSolution) -> Vec<i32> {
        let mut values = domain.to_vec();
        
        // Sort by nurse experience level (more experienced first for critical shifts)
        if let Some(shift_id) = extract_shift_id_from_var(var) {
            if let Some(&criticality) = self.shift_criticality.get(&shift_id) {
                if criticality >= CriticalityLevel::High {
                    values.sort_by_key(|&nurse_id| {
                        self.nurse_experience_levels.get(&(nurse_id as usize))
                            .map(|level| std::cmp::Reverse(*level))
                            .unwrap_or(std::cmp::Reverse(ExperienceLevel::Novice))
                    });
                }
            }
        }
        
        values
    }
    
    fn description(&self) -> String {
        "Healthcare scheduling: prioritize critical shifts, prefer experienced nurses".to_string()
    }
    
    fn priority(&self) -> u32 { 200 } // Higher than default
}

// Helper function
fn extract_shift_id_from_var(var: VarId) -> Option<usize> {
    // Extract shift ID from variable encoding
    todo!()
}
```

### Example 4: Integration Hook for External Systems

```rust
use serde_json;
use reqwest;

/// Integration hook for updating external HR system
struct HRSystemIntegration {
    api_endpoint: String,
    api_key: String,
    client: reqwest::Client,
}

impl HRSystemIntegration {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            api_endpoint: endpoint,
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

impl IntegrationHook for HRSystemIntegration {
    fn on_solution_found(&mut self, solution: &Solution) {
        // Convert solution to format expected by HR system
        let schedule_data = convert_solution_to_hr_format(solution);
        
        // Send async update (spawn task to avoid blocking solver)
        let client = self.client.clone();
        let endpoint = self.api_endpoint.clone();
        let api_key = self.api_key.clone();
        
        tokio::spawn(async move {
            let response = client
                .post(&format!("{}/schedule", endpoint))
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&schedule_data)
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    log::info!("Successfully updated HR system with new schedule");
                }
                Ok(resp) => {
                    log::error!("HR system update failed: {}", resp.status());
                }
                Err(e) => {
                    log::error!("Failed to send schedule to HR system: {}", e);
                }
            }
        });
    }
    
    fn on_solve_start(&mut self, model: &Model) {
        log::info!("Starting schedule optimization for {} variables", model.num_variables());
    }
    
    fn on_solve_complete(&mut self, result: &SolveResult) {
        match result {
            SolveResult::Optimal(solution) => {
                log::info!("Found optimal schedule with cost: {:.2}", solution.objective_value());
            }
            SolveResult::Feasible(solution) => {
                log::info!("Found feasible schedule with cost: {:.2}", solution.objective_value());
            }
            SolveResult::Infeasible => {
                log::warn!("No feasible schedule found - constraints too restrictive");
            }
            SolveResult::TimedOut => {
                log::warn!("Schedule optimization timed out");
            }
        }
    }
    
    fn description(&self) -> String {
        format!("HR System Integration ({})", self.api_endpoint)
    }
}

// Helper function
fn convert_solution_to_hr_format(solution: &Solution) -> serde_json::Value {
    // Convert internal solution format to HR system's expected JSON format
    todo!()
}
```

## Model Integration API

### Adding Custom Components to Model

```rust
impl Model {
    /// Add a custom constraint to the model
    pub fn add_custom_constraint<C>(&mut self, constraint: C) -> &mut Self 
    where 
        C: CustomConstraint + 'static 
    {
        self.custom_constraints.push(Box::new(constraint));
        self
    }
    
    /// Add a custom objective function
    pub fn add_custom_objective<O>(&mut self, objective: O) -> &mut Self 
    where 
        O: CustomObjective + 'static 
    {
        self.custom_objectives.push(Box::new(objective));
        self
    }
    
    /// Set a custom search heuristic
    pub fn set_custom_heuristic<H>(&mut self, heuristic: H) -> &mut Self 
    where 
        H: CustomHeuristic + 'static 
    {
        self.custom_heuristic = Some(Box::new(heuristic));
        self
    }
    
    /// Add a custom propagator
    pub fn add_custom_propagator<P>(&mut self, propagator: P) -> &mut Self 
    where 
        P: CustomPropagator + 'static 
    {
        self.custom_propagators.push(Box::new(propagator));
        self
    }
    
    /// Add an integration hook
    pub fn add_integration_hook<H>(&mut self, hook: H) -> &mut Self 
    where 
        H: IntegrationHook + 'static 
    {
        self.integration_hooks.push(Box::new(hook));
        self
    }
}
```

### Fluent Builder Pattern Usage

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut model = Model::default();
    
    // Create variables (employee scheduling example)
    let schedule_vars = create_schedule_variables(&mut model, &employees, &shifts);
    
    // Add standard constraints
    add_standard_constraints(&mut model, &schedule_vars, &requirements);
    
    // Add custom constraints and objectives
    model
        .add_custom_constraint(
            WorkloadBalanceConstraint::new(0.1)
                .with_weights(employee_weights)
        )
        .add_custom_constraint(
            UnionRulesConstraint::new()
                .max_consecutive_nights(3)
                .min_rest_hours(12)
        )
        .add_custom_objective(
            MinimizeCommuteCost::new(0.50)
                .add_employee_location(1, 40.7128, -74.0060)
                .add_workplace_location(1, 40.7589, -73.9851)
        )
        .set_custom_heuristic(
            HealthcareSchedulingHeuristic::new()
                .add_nurse_experience(1, ExperienceLevel::Senior)
                .add_shift_criticality(1, CriticalityLevel::Critical)
        )
        .add_integration_hook(
            HRSystemIntegration::new(
                "https://api.company.com/hr".to_string(),
                std::env::var("HR_API_KEY")?
            )
        )
        .add_integration_hook(
            SlackNotification::new("#scheduling".to_string())
        );
    
    // Solve with custom extensions
    match model.solve() {
        Ok(solution) => {
            println!("Found solution with cost: {:.2}", solution.objective_value());
            display_schedule(&solution);
        }
        Err(e) => {
            println!("Failed to find solution: {}", e);
        }
    }
    
    Ok(())
}
```

## Advanced Features

### 1. Multi-Objective Optimization

```rust
/// Weighted multi-objective optimization
struct MultiObjective {
    objectives: Vec<(Box<dyn CustomObjective>, f64)>, // (objective, weight)
}

impl MultiObjective {
    pub fn new() -> Self {
        Self { objectives: Vec::new() }
    }
    
    pub fn add_objective<O>(mut self, objective: O, weight: f64) -> Self 
    where 
        O: CustomObjective + 'static 
    {
        self.objectives.push((Box::new(objective), weight));
        self
    }
}

impl CustomObjective for MultiObjective {
    fn evaluate(&self, solution: &Solution) -> f64 {
        self.objectives.iter()
            .map(|(obj, weight)| {
                let value = obj.evaluate(solution);
                let normalized = if obj.is_minimizing() { value } else { -value };
                normalized * weight
            })
            .sum()
    }
    
    fn description(&self) -> String {
        format!("Multi-objective with {} components", self.objectives.len())
    }
}
```

### 2. Constraint Composition

```rust
/// Combine multiple constraints with logical operators
enum ConstraintComposition {
    And(Vec<Box<dyn CustomConstraint>>),
    Or(Vec<Box<dyn CustomConstraint>>),
    Not(Box<dyn CustomConstraint>),
    Implies(Box<dyn CustomConstraint>, Box<dyn CustomConstraint>),
}

impl CustomConstraint for ConstraintComposition {
    fn validate(&self, solution: &Solution) -> ConstraintResult {
        match self {
            Self::And(constraints) => {
                for constraint in constraints {
                    match constraint.validate(solution) {
                        ConstraintResult::Violated => return ConstraintResult::Violated,
                        ConstraintResult::Unknown => return ConstraintResult::Unknown,
                        _ => continue,
                    }
                }
                ConstraintResult::Satisfied
            }
            Self::Or(constraints) => {
                let mut all_violated = true;
                for constraint in constraints {
                    match constraint.validate(solution) {
                        ConstraintResult::Satisfied => return ConstraintResult::Satisfied,
                        ConstraintResult::Unknown => return ConstraintResult::Unknown,
                        ConstraintResult::Violated => continue,
                        ConstraintResult::SoftViolation(_) => all_violated = false,
                    }
                }
                if all_violated {
                    ConstraintResult::Violated
                } else {
                    ConstraintResult::Unknown
                }
            }
            // ... implement Not and Implies
            _ => todo!()
        }
    }
    
    fn description(&self) -> String {
        match self {
            Self::And(constraints) => {
                format!("AND({})", constraints.iter()
                    .map(|c| c.description())
                    .collect::<Vec<_>>()
                    .join(", "))
            }
            // ... other cases
            _ => todo!()
        }
    }
}
```

### 3. Performance Monitoring

```rust
/// Wrapper to monitor performance of custom constraints
struct MonitoredConstraint<C: CustomConstraint> {
    inner: C,
    call_count: AtomicUsize,
    total_duration: AtomicU64, // nanoseconds
}

impl<C: CustomConstraint> MonitoredConstraint<C> {
    pub fn new(constraint: C) -> Self {
        Self {
            inner: constraint,
            call_count: AtomicUsize::new(0),
            total_duration: AtomicU64::new(0),
        }
    }
    
    pub fn statistics(&self) -> (usize, Duration) {
        let calls = self.call_count.load(Ordering::Relaxed);
        let total_ns = self.total_duration.load(Ordering::Relaxed);
        (calls, Duration::from_nanos(total_ns))
    }
}

impl<C: CustomConstraint> CustomConstraint for MonitoredConstraint<C> {
    fn validate(&self, solution: &Solution) -> ConstraintResult {
        let start = Instant::now();
        let result = self.inner.validate(solution);
        let duration = start.elapsed();
        
        self.call_count.fetch_add(1, Ordering::Relaxed);
        self.total_duration.fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        
        result
    }
    
    fn description(&self) -> String {
        format!("Monitored({})", self.inner.description())
    }
    
    // Delegate other methods...
}
```

## Implementation Considerations

### 1. Performance Optimization

- **Trait Object Overhead**: Use `Box<dyn Trait>` only when dynamic dispatch is needed
- **Inline Hints**: Add `#[inline]` to frequently called trait methods
- **Specialization**: Use concrete types where possible for better optimization
- **Caching**: Cache expensive computations in custom implementations
- **Lazy Evaluation**: Only evaluate custom functions when necessary

### 2. Error Handling

```rust
/// Enhanced error handling for custom functions
#[derive(Debug, thiserror::Error)]
pub enum CustomFunctionError {
    #[error("Constraint validation failed: {message}")]
    ConstraintError { message: String },
    
    #[error("Objective evaluation failed: {message}")]
    ObjectiveError { message: String },
    
    #[error("Heuristic failed: {message}")]
    HeuristicError { message: String },
    
    #[error("Integration hook failed: {message}")]
    IntegrationError { message: String },
    
    #[error("External dependency unavailable: {dependency}")]
    DependencyError { dependency: String },
}

/// Result type for custom function operations
pub type CustomResult<T> = Result<T, CustomFunctionError>;
```

### 3. Thread Safety and Parallelization

```rust
/// Thread-safe constraint that can be used in parallel search
pub trait ParallelCustomConstraint: CustomConstraint + Send + Sync {
    /// Clone constraint for use in parallel thread
    fn clone_for_thread(&self) -> Box<dyn ParallelCustomConstraint>;
    
    /// Whether this constraint can be safely evaluated in parallel
    fn is_thread_safe(&self) -> bool { true }
}
```

### 4. Testing Framework

```rust
/// Testing utilities for custom constraints
pub mod testing {
    use super::*;
    
    /// Test harness for custom constraints
    pub struct ConstraintTester<C: CustomConstraint> {
        constraint: C,
        test_solutions: Vec<Solution>,
    }
    
    impl<C: CustomConstraint> ConstraintTester<C> {
        pub fn new(constraint: C) -> Self {
            Self {
                constraint,
                test_solutions: Vec::new(),
            }
        }
        
        pub fn add_test_solution(mut self, solution: Solution, expected: ConstraintResult) -> Self {
            self.test_solutions.push(solution);
            self
        }
        
        pub fn run_tests(&self) -> TestResults {
            // Run all test cases and collect results
            todo!()
        }
    }
}
```

## Questions for Further Discussion

1. **Performance vs. Flexibility**: How do we balance the overhead of trait objects with the flexibility they provide?

2. **Error Propagation**: Should custom function errors stop the search immediately, or should they be handled gracefully?

3. **Async Support**: Should we support async custom functions for external system integration?

4. **Validation**: How can we validate that custom constraints are correctly implemented?

5. **Documentation**: What level of documentation and examples do we need for each trait?

6. **Backwards Compatibility**: How do we evolve these traits while maintaining compatibility?

7. **Testing**: What testing utilities should we provide to help users validate their custom functions?

8. **Performance Monitoring**: Should performance monitoring be built-in or optional?

## Next Steps

1. **Core Trait Implementation**: Implement the basic trait definitions and model integration
2. **Example Library**: Create a library of common custom constraints and objectives
3. **Documentation**: Write comprehensive guides and API documentation
4. **Testing Framework**: Build tools to help users test their custom implementations
5. **Performance Benchmarking**: Establish benchmarks to measure custom function overhead
6. **Community Feedback**: Get feedback from potential users on the API design

## Conclusion

The trait-based system provides a powerful, type-safe, and performant way to extend Selen's capabilities. By leveraging Rust's trait system, we can offer flexibility while maintaining safety and performance guarantees. The modular design allows users to implement only the functionality they need while providing clear extension points for future enhancements.