# Efficient Float Solver Implementation Plan

## Overview
Transform the CSP solver from using inefficient binary search on float intervals to using optimal algorithms based on problem type classification. This addresses the core issue where simple problems like "maximize x < 5.5" take 287 propagations instead of 1 analytical step.

## Problem Analysis
- **Current issue**: Binary search treats float intervals like huge discrete domains (90K+ values for precision 4, 9M+ for precision 6)
- **Root cause**: One-size-fits-all search strategy doesn't leverage efficient FloatInterval operations
- **Solution**: Problem classification + algorithm selection based on constraint patterns

## Implementation Strategy

### **Phase 1: Foundation & Analysis (1-2 steps)**
1. **Problem Classification System** - Detect if a problem is pure float, pure integer, or mixed
2. **Benchmark Current Approach** - Establish baseline metrics for comparison

### **Phase 2: Pure Float Optimization (2-3 steps)**  
3. **Direct Bounds Optimization** - The core O(1) float optimization algorithm
4. **Integration with Existing Solver** - Hook into current solve/maximize methods
5. **Comprehensive Testing** - Verify performance gains

### **Phase 3: Mixed Problem Support (2-3 steps)**
6. **Separable Mixed Problems** - Handle independent float/int variables
7. **Basic MINLP Algorithm** - Branch-and-bound for coupled problems
8. **Performance Validation** - End-to-end benchmarking

## Directory Structure
```
src/
├── lib.rs
├── model.rs
├── domain/          # (existing)
├── search/          # (existing) 
├── optimization/    # (new) - efficient algorithms
│   ├── mod.rs
│   ├── classification.rs    # Problem type detection
│   ├── float_direct.rs      # O(1) float optimization  
│   ├── bounds_consistency.rs # Interval propagation
│   └── minlp.rs            # Mixed integer-float algorithms
└── benchmarks/      # (new) - performance testing
    ├── mod.rs
    └── comparison.rs        # Before/after metrics
```

## Step 1: Problem Classification System

### **What we're creating**
A system to automatically detect if a constraint problem is:
- **Pure float** (can use O(1) optimization)
- **Pure integer** (current solver works fine) 
- **Mixed with separable variables** (solve independently)
- **Mixed with coupled constraints** (needs MINLP)

### **Why this first**
Without classification, we can't route problems to the right algorithm. This is the foundation that makes everything else possible.

### **Implementation approach**
1. Create `/src/optimization/mod.rs` and `classification.rs`
2. Analyze the existing `Model` structure to understand how constraints are stored
3. Build a classifier that examines variable types and constraint patterns
4. Add simple tests to verify classification accuracy

### **Integration decisions**
- **Automatic classification** when `solve()`/`maximize()` is called (transparent to user)
- Classification happens internally, heuristics can be changed as new constraints are added
- Focus purely on classification logic, benchmarking comes later

## Expected Performance Improvements

### **Current Performance (Problematic Cases)**
- Simple float optimization: 287 propagations, 30 nodes, 1.5+ seconds
- Precision 6 problems: Hang indefinitely due to 9M+ step enumeration

### **Target Performance (After Implementation)**
- Pure float optimization: 1 analytical step, 0 nodes, <1ms
- Mixed problems: Integer search + O(1) float subproblems
- Precision 6 problems: Work correctly without hanging

## Academic Foundation
Based on established techniques:
- **Bounds Consistency**: Mackworth (1977), Waltz (1975)
- **Interval Arithmetic**: Moore (1966), Neumaier (1990)
- **MINLP Methods**: Grossmann & Kravanja (1997), Floudas (1995)
- **Industrial solvers**: CPLEX, Gurobi, SCIP, Choco-solver approaches

## Success Metrics
1. **Classification accuracy**: 100% correct problem type detection
2. **Performance gains**: >100x speedup for pure float problems
3. **Precision robustness**: All precision levels work without hanging
4. **Compatibility**: No breaking changes to existing API
5. **Mixed problem support**: Efficient handling of integer-float combinations

## Notes
- Existing `FloatInterval` operations (`remove_above`, `remove_below`, etc.) are perfect for efficient algorithms
- Current binary search works well for pure integer problems - keep it
- The key insight: problem classification enables algorithm selection
- Focus on making float problems fast, then extend to mixed cases
