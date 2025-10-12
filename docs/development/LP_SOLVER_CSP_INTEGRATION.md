# LP Solver CSP Integration Plan

## Overview

Integrate the LP solver with Selen's constraint solver to handle systems of linear float constraints efficiently, especially for large domains where interval propagation is slow.

## Current Situation

### Existing Float Linear Constraint Propagation
Selen currently has **interval-based propagators** for float linear constraints:
- `FloatLinEq`: `sum(coeffs[i] * vars[i]) = constant`
- `FloatLinLe`: `sum(coeffs[i] * vars[i]) ≤ constant`  
- `FloatLinNe`: `sum(coeffs[i] * vars[i]) ≠ constant`
- Reified versions of all above

**How they work:**
```rust
// For each variable x_i in constraint, compute bounds:
// remaining = constant - sum(coeff_j * x_j) for j ≠ i
// If coeff_i > 0: x_i ≤ remaining / coeff_i
// If coeff_i < 0: x_i ≥ remaining / coeff_i
```

### The Problem
For large float domains (e.g., ±1e6), interval propagation can be:
- **Slow**: O(n×m) per constraint, iterative refinement
- **Imprecise**: Interval arithmetic loses precision
- **Inefficient**: Doesn't find optimal solutions, just feasible regions

### The LP Solution
LP solver can handle entire system simultaneously:
- **Fast**: O(n³) worst-case for n variables, but finds optimal solution
- **Precise**: Exact arithmetic with controlled tolerance
- **Powerful**: Finds corner points, detects infeasibility/unboundedness

## Integration Strategy

### Phase 1: Detection & Extraction (Current Week)

**Goal**: Identify when LP solver should be invoked

#### 1.1 Linear Constraint Detection
Add detection logic in `Model` or search engine:
```rust
pub struct LinearConstraintSystem {
    variables: Vec<VarId>,           // Float variables involved
    constraints: Vec<LinearConstraint>,  // Extracted constraints
    objective: Option<Objective>,     // For optimization problems
}

struct LinearConstraint {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    relation: ConstraintRelation,     // Eq, Le, Ge
    rhs: f64,
}

enum ConstraintRelation {
    Equality,
    LessOrEqual,
    GreaterOrEqual,
}
```

**Detection triggers:**
- Multiple float linear constraints (≥ 3)
- Large domains (≥ 1000 possible values)
- Optimization problem with linear objective
- User hint/flag: `ModelConfig::use_lp_solver = true`

#### 1.2 Constraint Extraction
Scan `Propagators` to extract linear constraints:
```rust
impl Propagators {
    /// Extract linear constraints suitable for LP solving
    pub fn extract_linear_system(&self, vars: &Vars) -> Option<LinearConstraintSystem> {
        let mut system = LinearConstraintSystem::new();
        
        for prop in &self.props {
            match prop {
                Prop::FloatLinEq(p) => {
                    system.add_equality(p.coefficients, p.variables, p.constant);
                }
                Prop::FloatLinLe(p) => {
                    system.add_inequality(p.coefficients, p.variables, p.constant, Le);
                }
                // Handle other linear constraint types...
                _ => {
                    // Non-linear constraint found, might not use LP
                }
            }
        }
        
        if system.is_suitable_for_lp() {
            Some(system)
        } else {
            None
        }
    }
}
```

#### 1.3 Variable Bounds Extraction
Extract current bounds from Selen variables:
```rust
fn extract_variable_bounds(var: VarId, vars: &Vars) -> (f64, f64) {
    let lower = match var.min_via_vars(vars) {
        Val::ValF(f) => f,
        Val::ValI(i) => i as f64,
    };
    let upper = match var.max_via_vars(vars) {
        Val::ValF(f) => f,
        Val::ValI(i) => i as f64,
    };
    (lower, upper)
}
```

### Phase 2: LP Problem Construction

#### 2.1 Convert to LpProblem
```rust
impl LinearConstraintSystem {
    /// Convert to LP solver problem format
    pub fn to_lp_problem(&self, vars: &Vars, objective_sense: Maximize | Minimize) -> LpProblem {
        let n_vars = self.variables.len();
        let n_constraints = self.constraints.len();
        
        // Build objective vector
        let c = if let Some(obj) = &self.objective {
            obj.coefficients.clone()
        } else {
            vec![0.0; n_vars]  // Feasibility problem
        };
        
        // Build constraint matrix A and RHS b
        let mut a = Vec::with_capacity(n_constraints);
        let mut b = Vec::with_capacity(n_constraints);
        
        for constraint in &self.constraints {
            // Convert to standard form: all constraints as ≤
            match constraint.relation {
                Le => {
                    a.push(constraint.coefficients.clone());
                    b.push(constraint.rhs);
                }
                Ge => {
                    // x ≥ c  →  -x ≤ -c
                    a.push(constraint.coefficients.iter().map(|&c| -c).collect());
                    b.push(-constraint.rhs);
                }
                Eq => {
                    // x = c  →  x ≤ c AND x ≥ c (two constraints)
                    a.push(constraint.coefficients.clone());
                    b.push(constraint.rhs);
                    a.push(constraint.coefficients.iter().map(|&c| -c).collect());
                    b.push(-constraint.rhs);
                }
            }
        }
        
        // Extract variable bounds
        let lower_bounds: Vec<f64> = self.variables.iter()
            .map(|&v| extract_variable_bounds(v, vars).0)
            .collect();
            
        let upper_bounds: Vec<f64> = self.variables.iter()
            .map(|&v| extract_variable_bounds(v, vars).1)
            .collect();
        
        LpProblem::new(n_vars, a.len(), c, a, b, lower_bounds, upper_bounds)
    }
}
```

#### 2.2 Handle Special Cases
- **Unbounded variables**: Use `f64::NEG_INFINITY` / `f64::INFINITY`
- **Fixed variables**: Set lower = upper (LP solver handles this)
- **Integer variables**: Future work (currently LP only handles floats)

### Phase 3: Invocation & Integration

#### 3.1 When to Invoke LP Solver

**Option A: During Propagation** (Eager)
```rust
impl Context {
    pub fn propagate_with_lp(&mut self) -> Option<()> {
        // Run normal propagation first
        self.propagate()?;
        
        // Check if LP solving is beneficial
        if should_use_lp_solver(self) {
            let system = extract_linear_system(self)?;
            let lp_problem = system.to_lp_problem(&self.vars, Feasibility);
            
            match solve(&lp_problem) {
                Ok(solution) if solution.status == LpStatus::Optimal => {
                    // Update variable domains with LP solution
                    self.apply_lp_solution(&system, &solution)?;
                }
                Ok(solution) if solution.status == LpStatus::Infeasible => {
                    return None;  // Prune this branch
                }
                _ => {
                    // LP solver failed or unbounded, fall back to propagation
                }
            }
        }
        
        Some(())
    }
}
```

**Option B: During Search** (Lazy)
```rust
impl Engine {
    fn next(&mut self) -> Option<Solution> {
        // At each search node, check if LP can help
        if self.should_invoke_lp() {
            if let Some(lp_solution) = self.solve_lp_at_node() {
                // Use LP solution to guide branching
                self.branch_on_lp_solution(&lp_solution);
            }
        }
        // Continue normal search...
    }
}
```

**Option C: Hybrid** (Recommended)
- Use LP during initial propagation to tighten bounds
- Use LP at search nodes for complex problems
- Use interval propagation for simple constraints

#### 3.2 Apply LP Solution
```rust
fn apply_lp_solution(
    ctx: &mut Context,
    system: &LinearConstraintSystem,
    solution: &LpSolution,
) -> Option<()> {
    for (i, &var_id) in system.variables.iter().enumerate() {
        let lp_value = solution.x[i];
        
        // Tighten bounds based on LP solution
        // Option 1: Fix to LP solution (aggressive)
        var_id.try_set_min(Val::ValF(lp_value), ctx)?;
        var_id.try_set_max(Val::ValF(lp_value), ctx)?;
        
        // Option 2: Use LP solution as bound tightening (conservative)
        // (Would require sensitivity analysis or repeated LP solves)
    }
    
    Some(())
}
```

### Phase 4: Optimization Integration

#### 4.1 Linear Objective Functions
For optimization problems with linear objectives:
```rust
impl Model {
    pub fn minimize_linear(&mut self, coefficients: &[f64], variables: &[VarId]) {
        self.objective = Some(Objective::Linear {
            coefficients: coefficients.to_vec(),
            variables: variables.to_vec(),
            sense: Minimize,
        });
    }
}
```

#### 4.2 Solve as LP
```rust
impl Model {
    pub fn solve_with_lp(self) -> SolverResult<Solution> {
        // Extract linear system
        let system = self.props.extract_linear_system(&self.vars)?;
        
        // Convert to LP problem
        let lp_problem = system.to_lp_problem(&self.vars, self.objective_sense());
        
        // Solve with LP solver
        let lp_solution = solve(&lp_problem)?;
        
        // Convert back to CSP solution
        self.lp_solution_to_csp(system, lp_solution)
    }
}
```

### Phase 5: Performance Optimization

#### 5.1 Warm-Starting
For incremental solving (e.g., branch-and-bound):
```rust
struct LpCache {
    last_problem: LpProblem,
    last_solution: LpSolution,
}

impl Engine {
    fn solve_lp_warmstart(&mut self) -> Option<LpSolution> {
        if let Some(cache) = &self.lp_cache {
            // Reuse basis from previous solve
            solve_warmstart(&new_problem, &cache.last_solution, &config)
        } else {
            solve(&new_problem)
        }
    }
}
```

#### 5.2 Incremental Updates
Only re-solve when bounds change significantly:
```rust
fn should_resolve_lp(system: &LinearConstraintSystem, last_bounds: &[Interval]) -> bool {
    // Check if any bound changed by more than threshold
    system.variables.iter().enumerate().any(|(i, &var)| {
        let new_bounds = get_bounds(var);
        bounds_changed_significantly(&last_bounds[i], &new_bounds)
    })
}
```

## Implementation Checklist

### Week 4: CSP Integration (Current)

- [ ] **Detection Logic**
  - [ ] Implement `extract_linear_system()` in `Propagators`
  - [ ] Add `is_suitable_for_lp()` heuristics
  - [ ] Add `ModelConfig::prefer_lp_solver` flag

- [ ] **Conversion Layer**
  - [ ] Implement `LinearConstraintSystem` structure
  - [ ] Add `to_lp_problem()` conversion
  - [ ] Handle special cases (equalities, unbounded vars)

- [ ] **Integration Points**
  - [ ] Add LP invocation in propagation (Option A)
  - [ ] Implement `apply_lp_solution()` for bound updates
  - [ ] Add fallback to interval propagation

- [ ] **Testing**
  - [ ] Test on problems with 10-100 float variables
  - [ ] Compare performance: LP vs interval propagation
  - [ ] Test on infeasible problems
  - [ ] Test on unbounded problems

### Week 5: Optimization & Advanced Features

- [ ] **Objective Function Support**
  - [ ] Linear objective minimization/maximization
  - [ ] Integration with Selen's optimization API

- [ ] **Warm-Starting**
  - [ ] Implement `LpCache` for basis reuse
  - [ ] Measure warm-start speedup (target: 10x)

- [ ] **Hybrid Approach**
  - [ ] Automatic selection: LP vs propagation
  - [ ] Performance profiling and tuning

## Success Metrics

### Performance Goals
- **Speed**: 10-100x faster than interval propagation for 100+ variable problems
- **Accuracy**: LP solutions precise to within tolerance (1e-6)
- **Robustness**: Handle infeasible/unbounded cases gracefully
- **Memory**: Stay within configured limits

### Example Problems
1. **Production Planning**: 50 variables, 30 constraints (LP wins)
2. **Resource Allocation**: 100 variables, 80 constraints (LP wins)
3. **Simple Bounds**: 5 variables, 3 constraints (Propagation fine)
4. **Non-linear**: Mixed linear + non-linear (Hybrid)

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Selen Model                          │
│  - Variables (Float domains)                            │
│  - Constraints (Linear + others)                        │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  Constraint Solver   │
        └─────────┬───────────┘
                  │
         ┌────────┴─────────┐
         │                  │
         ▼                  ▼
┌────────────────┐  ┌──────────────────┐
│   Propagation  │  │  LP Solver       │
│   (Interval)   │  │  (Simplex)       │
└────────────────┘  └──────────────────┘
         │                  │
         └────────┬─────────┘
                  │
                  ▼
         ┌────────────────┐
         │   Solution     │
         │   (Tighter     │
         │    bounds)     │
         └────────────────┘
```

## Key Challenges

1. **Constraint Type Mixing**: Handle both linear (LP) and non-linear (propagation)
2. **Bound Synchronization**: Keep LP bounds in sync with CSP domains
3. **Performance Trade-offs**: When is LP actually faster?
4. **Numerical Stability**: Ensure LP and interval propagation agree
5. **API Design**: Make it seamless for users

## Next Steps

**Immediate (This Week)**:
1. Implement `extract_linear_system()` - parse existing propagators
2. Create conversion to `LpProblem` format
3. Add simple integration test: solve 10-variable LP within CSP

**Next Week**:
1. Benchmark: LP vs propagation on various problem sizes
2. Implement warm-starting for incremental solves
3. Add automatic LP/propagation selection heuristics

This integration will make Selen's float constraint solving dramatically faster for linear systems! 🚀
