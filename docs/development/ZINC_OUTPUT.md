# FlatZinc Output Format

## Overview
This document specifies how to format Selen solver results according to FlatZinc output conventions.

## FlatZinc Output Specification

### Satisfy Problems

#### Solution Found
```
x = 5;
y = 10;
----------
```

- Each variable assignment on a separate line
- Format: `<var_name> = <value>;`
- Solution separator: `----------`

#### Multiple Solutions
```
x = 5;
y = 10;
----------
x = 6;
y = 9;
----------
```

- Each solution separated by `----------`
- For `enumerate_all`: output all solutions, then `==========`

#### No Solution
```
=====UNSATISFIABLE=====
```

### Optimization Problems

#### Minimize
```
x = 5;
y = 10;
_objective = 15;
----------
x = 4;
y = 8;
_objective = 12;
==========
```

- Each improving solution is output
- `_objective` shows the objective value
- Final solution marked with `==========`

#### Maximize
Same format as minimize, but objective increases.

#### Unbounded or No Solution
```
=====UNBOUNDED=====
```
or
```
=====UNSATISFIABLE=====
```

### Output Variables

FlatZinc models specify which variables to output:
```
output ["x = ", show(x), ", y = ", show(y), "\n"];
```

**Handling:**
- Parse output specification from AST
- Format only specified variables
- If no output spec, output all variables

## Selen Integration

### Formatter Trait
```rust
// Pseudo-code
trait FlatZincFormatter {
    fn format_solution(&self, solution: &Solution, spec: &OutputSpec) -> String;
    fn format_unsatisfiable(&self) -> String;
    fn format_unbounded(&self) -> String;
}

struct DefaultFormatter;
impl FlatZincFormatter for DefaultFormatter { /* ... */ }
```

### API Design

#### For Library Use
```rust
// Function parameter for multiple solutions
pub fn solve_flatzinc(
    model_path: &Path,
    enumerate_all: bool,
) -> Result<Vec<Solution>, ZincError>;

// With custom formatter
pub fn solve_flatzinc_with_formatter(
    model_path: &Path,
    formatter: &dyn FlatZincFormatter,
    enumerate_all: bool,
) -> Result<String, ZincError>;
```

#### Output Example
```rust
let solutions = solve_flatzinc("model.fzn", true)?;
for solution in solutions {
    println!("{}", format_solution(&solution));
    println!("----------");
}
if !solutions.is_empty() {
    println!("==========");
}
```

## Variable Value Formatting

### Integer Variables
```
x = 42;
```

### Boolean Variables
```
b = true;
```

### Float Variables
```
f = 3.14159;
```

### Array Variables (1D)
```
arr = [1, 2, 3, 4, 5];
```

### Array Variables (2D)
```
matrix = [| 1, 2, 3 |
            4, 5, 6 |];
```

### Set Variables
```
s = {1, 3, 5, 7};
```

## Solver Statistics (Optional)

FlatZinc solvers often output statistics:
```
%%%mzn-stat: nodes=1234
%%%mzn-stat: failures=567
%%%mzn-stat: time=0.123
%%%mzn-stat-end
```

**Handling:**
- Optional feature (controlled by parameter)
- Use Selen's solver statistics if available
- Format: `%%%mzn-stat: <key>=<value>`

## Implementation Plan

1. Implement `FlatZincFormatter` trait and default implementation
2. Implement value formatting for each variable type
3. Handle output specifications from FlatZinc models
4. Add support for multiple solutions (enumerate_all)
5. Add optional solver statistics output
6. Test with FlatZinc examples

## Error Handling

### Invalid Output Specification
- Error: "Invalid output specification: ..."
- Fallback: Output all variables with default format

### Undefined Variable in Output Spec
- Error: "Variable 'x' in output spec is undefined"
- Fail with clear error message

## Open Questions

- Should we support custom output formats beyond FlatZinc spec?
- How to handle very large arrays (truncate or full output)?
- Should solver statistics be always included or opt-in?

## References

- [FlatZinc 2.8.4 Spec](https://docs.minizinc.dev/en/latest/fzn-spec.html) - Output section
- [MiniZinc Output Documentation](https://docs.minizinc.dev/en/stable/modelling.html#output)
