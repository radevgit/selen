# Review: Selen Missing Features Document

**Review Date**: October 4, 2025  
**Reviewer**: AI Assistant  
**Document Version**: Initial (October 4, 2025)

---

## Executive Summary

✅ **Document Quality**: Excellent, comprehensive, and well-structured  
✅ **Technical Accuracy**: Correctly identifies the gaps - **Zelen CAN parse FlatZinc, but Selen lacks the constraint methods**  
✅ **Actionability**: Clear implementation requirements with code examples  
✅ **Perspective**: Written from Zelen's viewpoint describing what's blocking full FlatZinc support  
⚠️ **Minor Gaps**: A few additional global constraints and edge cases to enumerate

---

## Strengths

### 1. Clear Problem Statement
- ✅ Excellent context: Zelen tested against ~900 files with 95% integer coverage
- ✅ Clear blocking issue: Float constraint support incomplete
- ✅ Quantifiable impact: Current workarounds are "broken" (loses precision, overflow)

### 2. Well-Prioritized Requirements
- ✅ P0/P1/P2 priority system is clear
- ✅ Focuses on critical blockers first (float_lin_eq, float_lin_le, float_lin_ne)
- ✅ Distinguishes between "must-have" and "nice-to-have"

### 3. Comprehensive API Design
- ✅ Consistent naming conventions with existing integer constraints
- ✅ Type safety considerations (f64 coefficients for float constraints)
- ✅ Error handling recommendations
- ✅ Full signature specifications for each method

### 4. FlatZinc Spec Compliance
- ✅ References FlatZinc Spec Section 4.2.3
- ✅ Notes MiniZinc 2.0 additions
- ✅ Aligns with standard builtins

### 5. Testing & Verification
- ✅ Specific test cases identified (loan.fzn, physics simulations)
- ✅ Verification commands provided
- ✅ Clear success criteria

---

## Identified Gaps & Recommendations

### Gap 1: Missing Global Constraints

**Issue**: The document mentions "probably some globals" in the initial context but doesn't enumerate them.

**Recommendation**: Add a section for missing **global constraints** commonly used in FlatZinc:

```markdown
## 11. Missing Global Constraints (Investigation Needed)

### Potentially Missing from Selen:

Based on FlatZinc 1.6+ specification, verify these globals:

#### Float Global Constraints:
- `all_different_float(array[int] of var float)` - Not in doc, may be missing
- `all_equal_float(array[int] of var float)` - Not in doc, may be missing

#### Set Constraints (if Selen supports set variables):
- `set_in` - Element in set membership
- `set_subset` - Set subset constraint
- `set_union` - Set union constraint
- `set_intersect` - Set intersection constraint

#### Table Constraints:
- `table_float` - Extension constraint for float variables
- Check if `table_int` is complete

#### Cumulative Constraints (Scheduling):
- `cumulative` - Resource scheduling constraint
- `diffn` - Non-overlapping 2D rectangles (packing)
- `circuit` - Hamiltonian circuit constraint

#### Regular Expression Constraints:
- `regular` - Regular language membership

**Action**: Audit Selen's implemented globals against FlatZinc 1.6 spec section 4.3
```

### Gap 2: Clarify Current Zelen Workaround

**Issue**: Section 8 describes the workaround but doesn't detail why it's fundamentally broken.

**Actually**: The document IS correct - this is about **Zelen's limitation** due to missing Selen methods.

**Current Situation**:
- ✅ Zelen CAN parse `float_lin_eq` from FlatZinc files
- ✅ Zelen's mapper knows these constraints exist
- ❌ Selen doesn't have `float_lin_eq()` method
- ⚠️ Zelen forced to scale floats → call `int_lin_eq()` → WRONG SEMANTICS

**The precision issue is in Zelen's workaround**, not Selen's representation.

**Document is CORRECT as-is**: Section 5 correctly notes that Selen has float variables with interval domains, so native float linear constraints should work properly once implemented.

### Gap 3: Optimization Integration

**Issue**: Mentions `float_direct.rs` exists but doesn't detail optimization requirements.

**Recommendation**: Add subsection:

```markdown
### 5.4 Float Optimization Requirements

**Current Selen Support** (verify):
- ✅ `Model::minimize(var)` works with float variables?
- ✅ `Model::maximize(var)` works with float variables?

**For Full FlatZinc Optimization**:

```rust
impl Model {
    /// Minimize float variable with optional search annotations
    pub fn minimize_float(&mut self, objective: VarId) -> Result<Solution, SolverError>;
    
    /// Maximize float variable with optional search annotations  
    pub fn maximize_float(&mut self, objective: VarId) -> Result<Solution, SolverError>;
}
```

**Search Annotations** (FlatZinc 1.6 Section 5):
- `float_search` annotation support
- Binary/linear search strategies for float domains
- Precision-based splitting
```

### Gap 4: Array Element Constraints

**Issue**: Only mentions `array_float_minimum/maximum`, but FlatZinc has more array constraints.

**Recommendation**: Complete the array constraint section:

```markdown
## 12. Float Array Constraints (P1 Priority)

### Missing from Selen:

```rust
impl Model {
    /// Float array minimum (already in doc)
    pub fn array_float_minimum(&mut self, result: VarId, array: &[VarId]);
    
    /// Float array maximum (already in doc)
    pub fn array_float_maximum(&mut self, result: VarId, array: &[VarId]);
    
    /// Float array element access
    /// result = array[index]
    pub fn array_float_element(&mut self, index: VarId, array: &[VarId], result: VarId);
}
```

**FlatZinc Usage**:
```flatzinc
var int: idx :: var_is_introduced = 1..10;
array[1..10] of var float: prices;
var float: selected_price;
constraint array_float_element(idx, prices, selected_price);
```

**Status**: `array_int_element` likely exists in Selen - need float version.
```

### Gap 5: Boolean/Integer ↔ Float Conversion

**Issue**: Not mentioned but needed for mixed constraint problems.

**Recommendation**: Add section:

```markdown
## 13. Type Conversion Constraints (P2 Priority)

### Missing from Selen:

```rust
impl Model {
    /// Convert integer variable to float
    /// float_var = int_var (implicit conversion)
    pub fn int2float(&mut self, int_var: VarId, float_var: VarId);
    
    /// Convert float to integer (floor/ceil/round)
    pub fn float2int_floor(&mut self, float_var: VarId, int_var: VarId);
    pub fn float2int_ceil(&mut self, float_var: VarId, int_var: VarId);
    pub fn float2int_round(&mut self, float_var: VarId, int_var: VarId);
}
```

**FlatZinc Spec**: Section 4.2.4 lists these as standard builtins.

**Use Case**:
```flatzinc
var float: weight = 67.5;
var int: rounded_weight;
constraint float2int_round(weight, rounded_weight);  % rounded_weight = 68
```
```

### Gap 6: Float Absolute Value

**Issue**: Not mentioned but common in optimization.

**Recommendation**:

```markdown
## 14. Float Arithmetic Constraints (P2 Priority)

### Check if Missing:

```rust
impl Model {
    /// Float absolute value: result = |x|
    pub fn float_abs(&mut self, x: VarId, result: VarId);
    
    /// Float power: result = x^n (integer exponent)
    pub fn float_pow(&mut self, x: VarId, n: i32, result: VarId);
    
    /// Float square root: result = sqrt(x)
    pub fn float_sqrt(&mut self, x: VarId, result: VarId);
}
```

**Note**: Selen may have `abs()` in runtime API - verify if it works with float variables.
```

### Gap 7: Performance Benchmarking

**Issue**: Mentions "2-3 days implementation" but no performance targets.

**Recommendation**: Add section:

```markdown
## 15. Performance Requirements & Benchmarks

### Target Performance:

**Small problems** (<100 float variables):
- ✅ Propagation: <1ms per constraint
- ✅ Solution time: <100ms

**Medium problems** (100-1000 float variables):
- ✅ Propagation: <10ms per constraint
- ✅ Solution time: <5 seconds

**Large problems** (>1000 float variables):
- ⚠️ May need optimization strategies
- Consider sparse matrix representations for large linear constraints

### Comparison Baseline:

Benchmark against existing solvers:
- **Gecode** (C++) - Fast float constraint propagation
- **Choco** (Java) - Reference implementation
- **OR-Tools** (C++) - Industry standard

**Zelen Integration**: Should be competitive within 2x of OR-Tools on float problems.
```

### Gap 8: Backwards Compatibility

**Issue**: Adding new methods to Selen API - what about version compatibility?

**Recommendation**: Add section:

```markdown
## 16. Version Compatibility & Migration

### Selen API Versioning:

**Current**: Selen v0.9.1 (without float linear constraints)  
**Proposed**: Selen v0.10.0 (with full float support)

**Semantic Versioning**:
- Minor version bump (0.9 → 0.10) since adding new methods
- Not breaking change for existing users
- Zelen will require `selen = ">=0.10.0"`

### Feature Detection in Zelen:

```rust
// Zelen can check Selen version at compile time
#[cfg(feature = "selen_float_linear")]
fn use_native_float_constraints() { ... }

#[cfg(not(feature = "selen_float_linear"))]
fn use_scaled_workaround() { ... }
```

**Migration Path**: Zelen v0.1.x works with Selen v0.9.x (scaled), Zelen v0.2.x requires Selen v0.10.x (native).
```

---

## Additional Considerations

### 1. Documentation Cross-References

**Add**: Links to specific Selen files that need modification:

```markdown
## 17. Implementation Roadmap for Selen Developers

### Files to Modify:

1. **`src/model/constraints.rs`**:
   - Add float linear constraint methods (~200 LOC)
   
2. **`src/constraints/propagators/`**:
   - New file: `float_linear.rs` (~500 LOC)
   - Implement interval arithmetic propagation
   
3. **`src/variables/domain/float_interval.rs`** (exists):
   - Verify interval operations are complete
   - Add methods: `intersect()`, `hull()`, `widen()`
   
4. **`src/core/constraint.rs`**:
   - Add enum variants for float constraints
   
5. **`tests/test_float_constraints.rs`** (new):
   - Unit tests for each constraint
   - Edge cases (NaN, Infinity, very small/large coefficients)
   
6. **`examples/float_optimization.rs`** (new):
   - Example showing float linear constraints in use

### Estimated LOC:
- Core implementation: ~1000 LOC
- Tests: ~500 LOC
- Documentation: ~200 LOC
- **Total**: ~1700 LOC for complete float support
```

### 2. Error Messages

**Add**: Requirements for user-friendly errors:

```markdown
## 18. Error Handling & User Experience

### Error Messages for Common Mistakes:

```rust
// When user provides wrong coefficient count
❌ Error: float_lin_eq requires coefficients.len() == variables.len()
   Got: 3 coefficients, 5 variables

// When user provides NaN coefficient
❌ Error: float_lin_eq coefficient at index 2 is NaN (not-a-number)
   Hint: Check your FlatZinc file for undefined float values

// When constraint is trivially infeasible
❌ Error: float_lin_eq is unsatisfiable
   Constraint: 1.0*x + 1.0*y = -5.0 with x,y ∈ [0.0, 10.0]
   Hint: Sum of positive variables cannot equal negative value
```

**Integration**: These errors should propagate cleanly to Zelen and ultimately to user.
```

### 3. FlatZinc Specification Compliance

**Add**: Explicit conformance checklist:

```markdown
## 19. FlatZinc 1.6 Compliance Checklist

### Float Constraints (Section 4.2.3):

- [ ] `float_lin_eq` - Linear equality
- [ ] `float_lin_le` - Linear less-or-equal
- [ ] `float_lin_ne` - Linear not-equal
- [ ] `float_lin_eq_reif` - Reified linear equality
- [ ] `float_lin_le_reif` - Reified linear less-or-equal
- [ ] `float_eq_reif` - Reified equality
- [ ] `float_ne_reif` - Reified not-equal
- [ ] `float_lt_reif` - Reified less-than
- [ ] `float_le_reif` - Reified less-or-equal
- [ ] `float_abs` - Absolute value
- [ ] `float_sqrt` - Square root
- [ ] `float_pow` - Power (integer exponent)
- [ ] `int2float` - Integer to float conversion
- [ ] `float2int` - Float to integer conversion

### Float Arrays (Section 4.2.3):

- [ ] `array_float_minimum` - Array minimum
- [ ] `array_float_maximum` - Array maximum
- [ ] `array_float_element` - Array element access

### Verification:

Run MiniZinc test suite against Selen+Zelen after implementation.
```

---

## Recommendations Summary

### Must Add to Document:

1. ✅ **Section 11**: Missing global constraints enumeration
2. ✅ **Section 5.3**: Precision and rounding strategy
3. ✅ **Section 12**: Complete float array constraints
4. ✅ **Section 13**: Type conversion constraints
5. ✅ **Section 17**: Implementation roadmap with file paths

### Should Add to Document:

6. ⚠️ **Section 15**: Performance requirements and benchmarks
7. ⚠️ **Section 16**: Version compatibility strategy
8. ⚠️ **Section 18**: Error handling requirements
9. ⚠️ **Section 19**: FlatZinc compliance checklist

### Nice to Have:

10. 💡 **Section 5.4**: Float optimization integration details
11. 💡 **Section 14**: Float arithmetic constraints (abs, sqrt, pow)

---

## Overall Assessment

**Rating**: ⭐⭐⭐⭐⭐ (5/5)

**Strengths**:
- Comprehensive coverage of P0/P1 priorities
- Excellent code examples and API design
- Clear actionable requirements
- Well-prioritized with impact assessment

**Minor Improvements Needed**:
- Enumerate missing global constraints explicitly
- Add precision/rounding strategy details
- Include implementation roadmap with file paths
- Add FlatZinc compliance checklist

**Recommendation**: 
- Document is **production-ready** for starting implementation
- Add suggested sections for completeness
- Use as specification document for Selen v0.10.0 development

---

## Next Steps

1. **For Selen Development Team**:
   - Review this document + review additions
   - Create GitHub issues for each P0 constraint
   - Assign implementation to sprint
   - Target: Selen v0.10.0 with float support

2. **For Zelen Development**:
   - Continue using scaled workaround for now
   - Prepare feature flags for native float constraints
   - Write integration tests ready for Selen v0.10.0

3. **Documentation**:
   - Incorporate review feedback into main document
   - Add to Selen repository as `FLOAT_CONSTRAINTS_SPEC.md`
   - Reference from Selen CHANGELOG when implemented

---

## Conclusion

The document is **excellent** and provides a clear roadmap for adding float constraint support to Selen. With the minor additions suggested above, it becomes a complete specification document that can guide implementation, testing, and verification.

The identified gaps (95% → 100% FlatZinc coverage) are clearly P0 priorities, and the proposed API is consistent with Selen's existing design patterns.

**Estimated Implementation Time** (revised): 
- P0 constraints: 3-4 days
- Full float support: 1-1.5 weeks
- Testing & documentation: 2-3 days
- **Total**: ~2 weeks for complete implementation

---

**Document Status**: ✅ APPROVED with recommended additions
