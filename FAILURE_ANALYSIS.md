# Failure Analysis - 65 Remaining Issues

## Category Breakdown

### 1. **Domain Size Exceeded** (21 files) ⚠️
Variables with domains > 10,000,000 elements.

**Files:**
- arrow.fzn: [123456789, 987654321] (864M elements)
- contains_array.fzn: [0, 999999999] (1B elements)
- curious_set_of_integers.fzn: [0, 100000000] (100M elements)
- digits_of_the_square.fzn: [1000000, 99980001] (98M elements)
- divisible_by_9_trough_1.fzn: [0, 999999999] (1B elements)
- enigma_1570.fzn: [0, 32461758] (32M elements)
- enigma_portuguese_squares.fzn: [1, 1000000000] (1B elements)
- euler_2.fzn: [0, 10000000] (10M elements)
- euler_9.fzn: [0, 125000000] (125M elements)
- four_power.fzn: [1, 64000000] (64M elements)
- grocery.fzn: [0, 359425431] (359M elements)
- magic_modulo_number.fzn: [0, 10000000] (10M elements)
- number_square.fzn: [110000000, 199999999] (90M elements)
- seven11.fzn: [0, 359425431] (359M elements)
- shopping_basket2.fzn: [6, 30000015] (30M elements)
- shopping_basket.fzn: [0, 10000435] (10M elements)
- square_root_of_wonderful.fzn: [123454321, 999999999] (876M elements)

**Solution:** Increase MAX_DOMAIN_SIZE or implement sparse domain representation

### 2. **Parse Errors** (17 files) 🔴
Parser issues with various constructs.

**Type A - "Expected Int, found IntLiteral(1)"** (11 files):
- bokus_competition.fzn (line 350)
- common.fzn (line 173)
- connected.fzn (line 78)
- in_set.fzn (line 1)
- open_alldifferent.fzn (line 4)
- open_among.fzn (line 11)
- open_atleast.fzn (line 9)
- open_atmost.fzn (line 9)
- open_global_cardinality.fzn (line 22)
- open_global_cardinality_low_up.fzn (line 23)
- roots_test.fzn (line 26)
- set_covering4b.fzn (line 2)

**Type B - "Expected RightBracket, found LeftParen"** (3 files):
- packing.fzn (line 20976)
- queens_viz.fzn (line 12)
- seg_fault.fzn (line 2736)

**Type C - Other** (3 files):
- debruijn_no_repetition.fzn: Expected RightParen, found LeftParen
- kaprekars_constant*.fzn (3 files): Unexpected token in expression: RightBrace
- regular_test.fzn: Lexical Error - Unexpected character: '/'

**Solution:** Fix parser to handle these edge cases

### 3. **SetLit in Array Elements** (15 files) 🟡
Array elements contain set literals like {1, 2, 3}.

**Files:**
- bus_scheduling_csplib.fzn
- combinatorial_auction.fzn
- exact_cover_dlx.fzn
- hitting_set.fzn
- itemset_mining.fzn
- map_coloring_with_costs.fzn
- maximal_independent_sets.fzn
- mixing_party.fzn
- number_of_regions.fzn
- optimal_picking_elements_from_each_list.fzn
- partial_latin_square.fzn
- sat.fzn
- scene_allocation.fzn
- set_covering2.fzn
- set_covering5.fzn

**Solution:** Support SetLit in array literal expressions in mapper.rs

### 4. **IntLit Mapping Errors** (6 files) 🟢 EASY FIX
"Expected variable identifier or array access, got: IntLit(X)"

**Files:**
- buckets.fzn: IntLit(0)
- einstein_opl.fzn: IntLit(1)
- houses.fzn: IntLit(1)
- lecture_series.fzn: IntLit(1)
- olympic.fzn: IntLit(3)
- timeslots_for_songs.fzn: IntLit(3)
- zebra_inverse.fzn: IntLit(1)

**Solution:** Find where get_var() is used instead of get_var_or_const()

### 5. **ArrayAccess in Array Elements** (2 files) 🟡
Array initialization contains array access like a[1].

**Files:**
- enigma_1575.fzn: ArrayAccess { array: "days", index: IntLit(3) }
- locker.fzn: ArrayAccess { array: "a_d1", index: IntLit(1) }

**Solution:** Support ArrayAccess in array literal expressions in mapper.rs

### 6. **Reified Constraint Issues** (2 files) 🟡
Unsupported argument types for reified constraints.

**Files:**
- battleships_6.fzn: int_eq_reif (line 1509)
- bug_unsat.fzn: int_ne_reif (line 7488)

**Solution:** Check and fix reified constraint mappers

### 7. **Unsupported Constraints** (1 file) 🟠
Not implemented.

**Files:**
- bobs_sale.fzn: global_cardinality_low_up_closed

**Solution:** Implement this global constraint

### 8. **Other Mapping Errors** (1 file) ��
Specific mapping issues.

**Files:**
- smullyan_lion_and_unicorn.fzn: Expected array literal in array_bool_element

**Solution:** Investigate specific case

---

## Priority Ranking

### 🟢 HIGH PRIORITY (Quick Wins - Est. +7 files)
1. **Fix IntLit mapping errors** (7 files)
   - Find remaining get_var() that should be get_var_or_const()
   - Estimated effort: 10 minutes
   - Impact: +7 files

### 🟡 MEDIUM PRIORITY (Est. +17 files)
2. **Support SetLit in arrays** (15 files)
   - Add SetLit case in array literal handling
   - Estimated effort: 30 minutes
   - Impact: +15 files

3. **Support ArrayAccess in arrays** (2 files)
   - Add ArrayAccess case in array literal handling
   - Estimated effort: 15 minutes
   - Impact: +2 files

### 🟠 LOW PRIORITY (Est. +3 files)
4. **Fix reified constraints** (2 files)
   - Debug int_eq_reif and int_ne_reif
   - Estimated effort: 30 minutes
   - Impact: +2 files

5. **Implement global_cardinality_low_up_closed** (1 file)
   - Estimated effort: 1 hour
   - Impact: +1 file

### ⚠️ DEFERRED (21 files - architectural decision needed)
6. **Domain size limit**
   - Requires architectural change (sparse domains or higher limit)
   - Estimated effort: 2-4 hours
   - Impact: +21 files

### 🔴 COMPLEX (17 files - parser work)
7. **Fix parser errors**
   - Requires parser fixes for edge cases
   - Estimated effort: 2-3 hours
   - Impact: +17 files

---

## Recommended Next Steps

**Option A: Quick Push to 95%** (Recommended)
1. Fix IntLit mapping errors → 797/851 (93.7%)
2. Support SetLit in arrays → 812/851 (95.4%) ✅ **95% ACHIEVED!**

**Option B: Maximum Impact**
1. Do Option A (Quick push to 95%)
2. Support ArrayAccess in arrays → 814/851 (95.7%)
3. Fix reified constraints → 816/851 (95.9%)

**Option C: Complete Coverage** (Long term)
- All of Option B
- Implement missing constraint
- Fix parser errors
- Address domain size limit
→ Target: 851/851 (100%)

