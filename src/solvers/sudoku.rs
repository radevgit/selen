//! Specialized Sudoku solver with optimized constraints and human-solving techniques.
//!
//! This module provides a production-ready Sudoku solver that can be easily used
//! without manual constraint setup. It includes both basic constraint propagation
//! and can be extended with advanced human-solving techniques for better performance.

use crate::model::*;
use crate::prelude::*;
use crate::runtime_api::*;
use std::time::Instant;

/// A sparse set optimized for Sudoku candidate tracking (domain 1-9).
/// Much faster than HashSet for this specific use case.
#[derive(Debug, Clone, PartialEq)]
pub struct SudokuCandidateSet {
    /// Bit mask where bit i represents digit i+1 (bit 0 = digit 1, bit 8 = digit 9)
    mask: u16,
}

impl SudokuCandidateSet {
    /// Create a new empty candidate set
    pub fn new() -> Self {
        Self { mask: 0 }
    }
    
    /// Create a candidate set with all digits 1-9
    pub fn full() -> Self {
        Self { mask: 0b111111111 } // bits 0-8 set
    }
    
    /// Create a candidate set with a single digit
    pub fn single(digit: i32) -> Self {
        debug_assert!(digit >= 1 && digit <= 9);
        Self { mask: 1 << (digit - 1) }
    }
    
    /// Insert a digit into the candidate set
    pub fn insert(&mut self, digit: i32) {
        debug_assert!(digit >= 1 && digit <= 9);
        self.mask |= 1 << (digit - 1);
    }
    
    /// Remove a digit from the candidate set, returns true if it was present
    pub fn remove(&mut self, digit: i32) -> bool {
        debug_assert!(digit >= 1 && digit <= 9);
        let bit = 1 << (digit - 1);
        let was_present = (self.mask & bit) != 0;
        self.mask &= !bit;
        was_present
    }
    
    /// Check if a digit is in the candidate set
    pub fn contains(&self, digit: i32) -> bool {
        debug_assert!(digit >= 1 && digit <= 9);
        (self.mask & (1 << (digit - 1))) != 0
    }
    
    /// Get the number of candidates
    pub fn len(&self) -> usize {
        self.mask.count_ones() as usize
    }
    
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }
    
    /// Get the single candidate if there's exactly one, otherwise None
    pub fn single_candidate(&self) -> Option<i32> {
        if self.len() == 1 {
            Some((self.mask.trailing_zeros() + 1) as i32)
        } else {
            None
        }
    }
    
    /// Iterate over all candidates
    pub fn iter(&self) -> SudokuCandidateIter {
        SudokuCandidateIter { mask: self.mask, current: 0 }
    }
    
    /// Clear all candidates
    pub fn clear(&mut self) {
        self.mask = 0;
    }
}

impl Default for SudokuCandidateSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::iter::IntoIterator for &SudokuCandidateSet {
    type Item = i32;
    type IntoIter = SudokuCandidateIter;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over candidates in a SudokuCandidateSet
#[derive(Debug)]
pub struct SudokuCandidateIter {
    mask: u16,
    current: u8,
}

impl Iterator for SudokuCandidateIter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        while self.current < 9 {
            if (self.mask & (1 << self.current)) != 0 {
                let digit = (self.current + 1) as i32;
                self.current += 1;
                return Some(digit);
            }
            self.current += 1;
        }
        None
    }
}

/// A specialized Sudoku solver with clean API and optimized performance.
#[derive(Debug)]
pub struct SudokuSolver {
    model: Model,
    grid: Vec<Vec<VarId>>,
    original_puzzle: [[i32; 9]; 9],
    /// Candidate tracking for each cell using optimized sparse sets
    candidates: Vec<Vec<SudokuCandidateSet>>,
}

/// Result of a Sudoku solving attempt with detailed statistics.
#[derive(Debug, Clone)]
pub struct SudokuResult {
    /// The solved grid, or None if no solution exists
    pub solution: Option<[[i32; 9]; 9]>,
    /// Number of constraint propagations performed
    pub propagations: usize,
    /// Number of search nodes explored
    pub nodes: usize,
    /// Time taken to solve in milliseconds
    pub duration_ms: f64,
    /// Whether the solution was found purely by constraint propagation
    pub pure_propagation: bool,
}

impl SudokuSolver {
    /// Create a new Sudoku solver for the given puzzle.
    /// 
    /// # Arguments
    /// * `puzzle` - A 9x9 grid where 0 represents empty cells and 1-9 are clues
    /// 
    /// # Example
    /// ```
    /// use selen::solvers::SudokuSolver;
    /// 
    /// let puzzle = [
    ///     [5, 3, 0, 0, 7, 0, 0, 0, 0],
    ///     [6, 0, 0, 1, 9, 5, 0, 0, 0],
    ///     [0, 9, 8, 0, 0, 0, 0, 6, 0],
    ///     [8, 0, 0, 0, 6, 0, 0, 0, 3],
    ///     [4, 0, 0, 8, 0, 3, 0, 0, 1],
    ///     [7, 0, 0, 0, 2, 0, 0, 0, 6],
    ///     [0, 6, 0, 0, 0, 0, 2, 8, 0],
    ///     [0, 0, 0, 4, 1, 9, 0, 0, 5],
    ///     [0, 0, 0, 0, 8, 0, 0, 7, 9],
    /// ];
    /// let solver = SudokuSolver::new(puzzle);
    /// let result = solver.solve();
    /// ```
    pub fn new(puzzle: [[i32; 9]; 9]) -> Self {
        let mut model = Model::default();
        let mut grid = Vec::new();
        let mut candidates = Vec::new();
        
        // Create variables and initialize candidates for each cell
        for row in 0..9 {
            let mut grid_row = Vec::new();
            let mut candidate_row = Vec::new();
            
            for col in 0..9 {
                let var = if puzzle[row][col] != 0 {
                    // Clue: create singleton variable
                    let clue_val = puzzle[row][col];
                    model.int(clue_val, clue_val)
                } else {
                    // Empty cell: domain 1-9
                    model.int(1, 9)
                };
                grid_row.push(var);
                
                // Initialize candidates using optimized sparse set
                let cell_candidates = if puzzle[row][col] == 0 {
                    // Empty cell: all digits 1-9 are initially possible
                    SudokuCandidateSet::full()
                } else {
                    // Clue: only the given digit is possible
                    SudokuCandidateSet::single(puzzle[row][col])
                };
                candidate_row.push(cell_candidates);
            }
            grid.push(grid_row);
            candidates.push(candidate_row);
        }
        
        let mut solver = SudokuSolver {
            model,
            grid,
            original_puzzle: puzzle,
            candidates,
        };
        
        // Add standard Sudoku constraints
        solver.add_basic_constraints();
        
        // Update candidates based on initial constraints
        solver.update_candidates();
        
        solver
    }
    
    /// Add the basic Sudoku constraints (rows, columns, boxes).
    fn add_basic_constraints(&mut self) {
        // Row constraints - each row must contain all digits 1-9
        for row in 0..9 {
            self.model.alldiff(&self.grid[row]);
        }
        
        // Column constraints - each column must contain all digits 1-9
        for col in 0..9 {
            let column_vars: Vec<VarId> = (0..9).map(|row| self.grid[row][col]).collect();
            self.model.alldiff(&column_vars);
        }
        
        // Box constraints - each 3x3 box must contain all digits 1-9
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut box_vars = Vec::with_capacity(9);
                for r in 0..3 {
                    for c in 0..3 {
                        box_vars.push(self.grid[box_row * 3 + r][box_col * 3 + c]);
                    }
                }
                self.model.alldiff(&box_vars);
            }
        }
    }
    
    /// Update candidate tracking based on current state.
    /// This removes impossible candidates from empty cells based on row/column/box constraints.
    fn update_candidates(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                if self.original_puzzle[row][col] == 0 {
                    // Empty cell - update candidates based on constraints
                    let mut new_candidates = SudokuCandidateSet::new();
                    for digit in 1..=9 {
                        if self.is_candidate_valid(row, col, digit) {
                            new_candidates.insert(digit);
                        }
                    }
                    self.candidates[row][col] = new_candidates;
                }
            }
        }
    }
    
    /// Check if a digit is a valid candidate for a cell based on Sudoku rules.
    fn is_candidate_valid(&self, row: usize, col: usize, digit: i32) -> bool {
        // Check row - digit shouldn't appear in same row
        for c in 0..9 {
            if c != col && self.original_puzzle[row][c] == digit {
                return false;
            }
        }
        
        // Check column - digit shouldn't appear in same column
        for r in 0..9 {
            if r != row && self.original_puzzle[r][col] == digit {
                return false;
            }
        }
        
        // Check 3x3 box - digit shouldn't appear in same box
        let box_row_start = (row / 3) * 3;
        let box_col_start = (col / 3) * 3;
        for r in box_row_start..box_row_start + 3 {
            for c in box_col_start..box_col_start + 3 {
                if (r != row || c != col) && self.original_puzzle[r][c] == digit {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Apply advanced Sudoku solving techniques to add dynamic constraints.
    /// Returns true if any new constraints were added.
    pub fn apply_advanced_techniques(&mut self) -> bool {
        let mut made_progress = false;
        
        // Apply the three most effective techniques based on benchmarking
        made_progress |= self.apply_naked_singles();
        made_progress |= self.apply_hidden_singles();
        made_progress |= self.apply_naked_pairs();
        
        if made_progress {
            self.update_candidates();
        }
        
        made_progress
    }
    
    /// Apply naked singles technique: if a cell has only one candidate, assign it.
    fn apply_naked_singles(&mut self) -> bool {
        let mut progress = false;
        
        for row in 0..9 {
            for col in 0..9 {
                if self.original_puzzle[row][col] == 0 && self.candidates[row][col].len() == 1 {
                    let digit = self.candidates[row][col].single_candidate().unwrap();
                    
                    // Add constraint that this cell must equal the digit
                    self.model.props.equals(self.grid[row][col], Val::int(digit));
                    progress = true;
                }
            }
        }
        
        progress
    }
    
    /// Apply hidden singles technique: if a digit has only one possible position in a unit.
    fn apply_hidden_singles(&mut self) -> bool {
        let mut progress = false;
        
        // Check rows
        for row in 0..9 {
            for digit in 1..=9 {
                let mut possible_positions = Vec::new();
                for col in 0..9 {
                    if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                        possible_positions.push(col);
                    }
                }
                
                if possible_positions.len() == 1 {
                    let col = possible_positions[0];
                    self.model.props.equals(self.grid[row][col], Val::int(digit));
                    progress = true;
                }
            }
        }
        
        // Check columns
        for col in 0..9 {
            for digit in 1..=9 {
                let mut possible_positions = Vec::new();
                for row in 0..9 {
                    if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                        possible_positions.push(row);
                    }
                }
                
                if possible_positions.len() == 1 {
                    let row = possible_positions[0];
                    self.model.props.equals(self.grid[row][col], Val::int(digit));
                    progress = true;
                }
            }
        }
        
        // Check 3x3 boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                for digit in 1..=9 {
                    let mut possible_positions = Vec::new();
                    for r in 0..3 {
                        for c in 0..3 {
                            let row = box_row * 3 + r;
                            let col = box_col * 3 + c;
                            if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                                possible_positions.push((row, col));
                            }
                        }
                    }
                    
                    if possible_positions.len() == 1 {
                        let (row, col) = possible_positions[0];
                        self.model.props.equals(self.grid[row][col], Val::int(digit));
                        progress = true;
                    }
                }
            }
        }
        
        progress
    }
    
    /// Apply box/line reduction (pointing pairs/triples): if a digit can only appear
    /// in one row/column within a box, eliminate it from that row/column outside the box.
    fn apply_box_line_reduction(&mut self) -> bool {
        let mut progress = false;
        
        // Check each 3x3 box
        for box_row in 0..3 {
            for box_col in 0..3 {
                for digit in 1..=9 {
                    // Find all positions where this digit can appear in this box
                    let mut positions = Vec::new();
                    for r in 0..3 {
                        for c in 0..3 {
                            let row = box_row * 3 + r;
                            let col = box_col * 3 + c;
                            if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                                positions.push((row, col));
                            }
                        }
                    }
                    
                    if positions.len() >= 2 {
                        // Check if all positions are in the same row
                        let first_row = positions[0].0;
                        if positions.iter().all(|(row, _)| *row == first_row) {
                            // All positions in same row - eliminate digit from this row outside the box
                            for col in 0..9 {
                                let col_box = col / 3;
                                if col_box != box_col && self.original_puzzle[first_row][col] == 0 {
                                    if self.candidates[first_row][col].remove(digit) {
                                        progress = true;
                                    }
                                }
                            }
                        }
                        
                        // Check if all positions are in the same column
                        let first_col = positions[0].1;
                        if positions.iter().all(|(_, col)| *col == first_col) {
                            // All positions in same column - eliminate digit from this column outside the box
                            for row in 0..9 {
                                let row_box = row / 3;
                                if row_box != box_row && self.original_puzzle[row][first_col] == 0 {
                                    if self.candidates[row][first_col].remove(digit) {
                                        progress = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        progress
    }
    
    /// Apply X-Wing pattern: if a digit forms a rectangle in exactly 2 rows and 2 columns,
    /// eliminate it from those rows/columns elsewhere.
    fn apply_x_wing(&mut self) -> bool {
        let mut progress = false;
        
        // Check for X-Wing patterns in rows
        for digit in 1..=9 {
            // Find rows where digit has exactly 2 possible positions
            let mut candidate_rows = Vec::new();
            for row in 0..9 {
                let mut positions = Vec::new();
                for col in 0..9 {
                    if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                        positions.push(col);
                    }
                }
                if positions.len() == 2 {
                    candidate_rows.push((row, positions));
                }
            }
            
            // Look for X-Wing pattern: two rows with digit in same two columns
            for i in 0..candidate_rows.len() {
                for j in i + 1..candidate_rows.len() {
                    let (row1, positions1) = &candidate_rows[i];
                    let (row2, positions2) = &candidate_rows[j];
                    
                    if positions1 == positions2 {
                        // Found X-Wing! Eliminate digit from these columns in other rows
                        let col1 = positions1[0];
                        let col2 = positions1[1];
                        
                        for row in 0..9 {
                            if row != *row1 && row != *row2 {
                                if self.original_puzzle[row][col1] == 0 {
                                    if self.candidates[row][col1].remove(digit) {
                                        progress = true;
                                    }
                                }
                                if self.original_puzzle[row][col2] == 0 {
                                    if self.candidates[row][col2].remove(digit) {
                                        progress = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Check for X-Wing patterns in columns (transpose of row logic)
        for digit in 1..=9 {
            // Find columns where digit has exactly 2 possible positions
            let mut candidate_cols = Vec::new();
            for col in 0..9 {
                let mut positions = Vec::new();
                for row in 0..9 {
                    if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                        positions.push(row);
                    }
                }
                if positions.len() == 2 {
                    candidate_cols.push((col, positions));
                }
            }
            
            // Look for X-Wing pattern: two columns with digit in same two rows
            for i in 0..candidate_cols.len() {
                for j in i + 1..candidate_cols.len() {
                    let (col1, positions1) = &candidate_cols[i];
                    let (col2, positions2) = &candidate_cols[j];
                    
                    if positions1 == positions2 {
                        // Found X-Wing! Eliminate digit from these rows in other columns
                        let row1 = positions1[0];
                        let row2 = positions1[1];
                        
                        for col in 0..9 {
                            if col != *col1 && col != *col2 {
                                if self.original_puzzle[row1][col] == 0 {
                                    if self.candidates[row1][col].remove(digit) {
                                        progress = true;
                                    }
                                }
                                if self.original_puzzle[row2][col] == 0 {
                                    if self.candidates[row2][col].remove(digit) {
                                        progress = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        progress
    }
    
    /// Apply naked pairs technique: if two cells in a unit have the same two candidates,
    /// eliminate those candidates from other cells in the unit.
    fn apply_naked_pairs(&mut self) -> bool {
        let mut progress = false;
        
        // Check rows for naked pairs
        for row in 0..9 {
            progress |= self.find_naked_pairs_in_row(row);
        }
        
        // Check columns for naked pairs
        for col in 0..9 {
            progress |= self.find_naked_pairs_in_column(col);
        }
        
        // Check boxes for naked pairs
        for box_row in 0..3 {
            for box_col in 0..3 {
                progress |= self.find_naked_pairs_in_box(box_row, box_col);
            }
        }
        
        progress
    }
    
    /// Find naked pairs in a specific row.
    fn find_naked_pairs_in_row(&mut self, row: usize) -> bool {
        let mut progress = false;
        
        for col1 in 0..8 {
            if self.original_puzzle[row][col1] != 0 || self.candidates[row][col1].len() != 2 {
                continue;
            }
            
            for col2 in col1 + 1..9 {
                if self.original_puzzle[row][col2] != 0 || self.candidates[row][col2].len() != 2 {
                    continue;
                }
                
                if self.candidates[row][col1] == self.candidates[row][col2] {
                    // Found naked pair - eliminate these candidates from other cells in row
                    let pair_candidates: Vec<i32> = self.candidates[row][col1].iter().collect();
                    
                    for col in 0..9 {
                        if col != col1 && col != col2 && self.original_puzzle[row][col] == 0 {
                            for &digit in &pair_candidates {
                                if self.candidates[row][col].remove(digit) {
                                    progress = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        progress
    }
    
    /// Find naked pairs in a specific column.
    fn find_naked_pairs_in_column(&mut self, col: usize) -> bool {
        let mut progress = false;
        
        for row1 in 0..8 {
            if self.original_puzzle[row1][col] != 0 || self.candidates[row1][col].len() != 2 {
                continue;
            }
            
            for row2 in row1 + 1..9 {
                if self.original_puzzle[row2][col] != 0 || self.candidates[row2][col].len() != 2 {
                    continue;
                }
                
                if self.candidates[row1][col] == self.candidates[row2][col] {
                    // Found naked pair - eliminate these candidates from other cells in column
                    let pair_candidates: Vec<i32> = self.candidates[row1][col].iter().collect();
                    
                    for row in 0..9 {
                        if row != row1 && row != row2 && self.original_puzzle[row][col] == 0 {
                            for &digit in &pair_candidates {
                                if self.candidates[row][col].remove(digit) {
                                    progress = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        progress
    }
    
    /// Find naked pairs in a specific 3x3 box.
    fn find_naked_pairs_in_box(&mut self, box_row: usize, box_col: usize) -> bool {
        let mut progress = false;
        let mut cells = Vec::new();
        
        // Collect all empty cells in the box
        for r in 0..3 {
            for c in 0..3 {
                let row = box_row * 3 + r;
                let col = box_col * 3 + c;
                if self.original_puzzle[row][col] == 0 {
                    cells.push((row, col));
                }
            }
        }
        
        // Look for naked pairs
        for i in 0..cells.len() {
            let (row1, col1) = cells[i];
            if self.candidates[row1][col1].len() != 2 {
                continue;
            }
            
            for j in i + 1..cells.len() {
                let (row2, col2) = cells[j];
                if self.candidates[row2][col2].len() != 2 {
                    continue;
                }
                
                if self.candidates[row1][col1] == self.candidates[row2][col2] {
                    // Found naked pair - eliminate these candidates from other cells in box
                    let pair_candidates: Vec<i32> = self.candidates[row1][col1].iter().collect();
                    
                    for &(row, col) in &cells {
                        if (row, col) != (row1, col1) && (row, col) != (row2, col2) {
                            for &digit in &pair_candidates {
                                if self.candidates[row][col].remove(digit) {
                                    progress = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        progress
    }
    
    /// Apply Alternating Inference Chain (AIC) technique: build chains of logical
    /// implications to find contradictions and make eliminations.
    fn apply_alternating_inference_chains(&mut self) -> bool {
        let mut progress = false;
        
        // Try to build chains for each digit
        for digit in 1..=9 {
            progress |= self.build_aic_chains_for_digit(digit);
        }
        
        progress
    }
    
    /// Build AIC chains for a specific digit to find eliminations.
    fn build_aic_chains_for_digit(&mut self, digit: i32) -> bool {
        let mut progress = false;
        
        // Find all candidate positions for this digit
        let mut candidate_positions = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                if self.original_puzzle[row][col] == 0 && self.candidates[row][col].contains(digit) {
                    candidate_positions.push((row, col));
                }
            }
        }
        
        // Try building chains starting from each position
        for &start_pos in &candidate_positions {
            let mut visited = std::collections::HashSet::new();
            let mut chain = Vec::new();
            
            if self.build_chain(digit, start_pos, true, &mut visited, &mut chain, 0) {
                progress = true;
                break; // Found a useful chain, apply it and restart
            }
        }
        
        progress
    }
    
    /// Recursively build an alternating inference chain.
    /// Returns true if a contradiction or useful elimination was found.
    fn build_chain(
        &mut self,
        digit: i32,
        pos: (usize, usize),
        is_strong_link: bool,
        visited: &mut std::collections::HashSet<(usize, usize)>,
        chain: &mut Vec<((usize, usize), bool)>,
        depth: usize,
    ) -> bool {
        // Limit chain depth to prevent infinite recursion
        if depth > 6 || visited.contains(&pos) {
            return false;
        }
        
        visited.insert(pos);
        chain.push((pos, is_strong_link));
        
        // If we've built a chain of reasonable length, look for contradictions
        if chain.len() >= 4 {
            if let Some(elimination) = self.find_aic_elimination(digit, chain) {
                // Apply the elimination
                let (elim_row, elim_col) = elimination;
                if self.candidates[elim_row][elim_col].remove(digit) {
                    visited.remove(&pos);
                    chain.pop();
                    return true;
                }
            }
        }
        
        // Continue building the chain
        let next_positions = if is_strong_link {
            // Strong link: find positions that force this digit elsewhere
            self.find_weak_links(digit, pos)
        } else {
            // Weak link: find positions that exclude this digit
            self.find_strong_links(digit, pos)
        };
        
        for next_pos in next_positions {
            if self.build_chain(digit, next_pos, !is_strong_link, visited, chain, depth + 1) {
                visited.remove(&pos);
                chain.pop();
                return true;
            }
        }
        
        visited.remove(&pos);
        chain.pop();
        false
    }
    
    /// Find strong links for AIC: positions where if one is false, another must be true.
    fn find_strong_links(&self, digit: i32, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut links = Vec::new();
        let (row, col) = pos;
        
        // Check row: if only 2 positions in row can have this digit
        let mut row_candidates = Vec::new();
        for c in 0..9 {
            if self.original_puzzle[row][c] == 0 && self.candidates[row][c].contains(digit) {
                row_candidates.push((row, c));
            }
        }
        if row_candidates.len() == 2 {
            for &candidate_pos in &row_candidates {
                if candidate_pos != pos {
                    links.push(candidate_pos);
                }
            }
        }
        
        // Check column: if only 2 positions in column can have this digit
        let mut col_candidates = Vec::new();
        for r in 0..9 {
            if self.original_puzzle[r][col] == 0 && self.candidates[r][col].contains(digit) {
                col_candidates.push((r, col));
            }
        }
        if col_candidates.len() == 2 {
            for &candidate_pos in &col_candidates {
                if candidate_pos != pos {
                    links.push(candidate_pos);
                }
            }
        }
        
        // Check box: if only 2 positions in box can have this digit
        let box_row_start = (row / 3) * 3;
        let box_col_start = (col / 3) * 3;
        let mut box_candidates = Vec::new();
        for r in box_row_start..box_row_start + 3 {
            for c in box_col_start..box_col_start + 3 {
                if self.original_puzzle[r][c] == 0 && self.candidates[r][c].contains(digit) {
                    box_candidates.push((r, c));
                }
            }
        }
        if box_candidates.len() == 2 {
            for &candidate_pos in &box_candidates {
                if candidate_pos != pos {
                    links.push(candidate_pos);
                }
            }
        }
        
        links
    }
    
    /// Find weak links for AIC: positions that see each other (can't both be true).
    fn find_weak_links(&self, digit: i32, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut links = Vec::new();
        let (row, col) = pos;
        
        // All positions that see this position (same row, column, or box)
        for r in 0..9 {
            for c in 0..9 {
                if (r, c) != pos 
                    && self.original_puzzle[r][c] == 0 
                    && self.candidates[r][c].contains(digit)
                    && self.positions_see_each_other((row, col), (r, c)) {
                    links.push((r, c));
                }
            }
        }
        
        links
    }
    
    /// Check if two positions "see" each other (same row, column, or box).
    fn positions_see_each_other(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        let (row1, col1) = pos1;
        let (row2, col2) = pos2;
        
        // Same row
        if row1 == row2 {
            return true;
        }
        
        // Same column
        if col1 == col2 {
            return true;
        }
        
        // Same box
        if (row1 / 3) == (row2 / 3) && (col1 / 3) == (col2 / 3) {
            return true;
        }
        
        false
    }
    
    /// Find eliminations based on AIC chain analysis.
    fn find_aic_elimination(&self, digit: i32, chain: &[((usize, usize), bool)]) -> Option<(usize, usize)> {
        if chain.len() < 4 {
            return None;
        }
        
        // Look for chain patterns that create contradictions
        let start_pos = chain[0].0;
        let end_pos = chain[chain.len() - 1].0;
        
        // If start and end positions see each other, we have a contradiction loop
        if self.positions_see_each_other(start_pos, end_pos) {
            // Find positions that see both start and end - they can be eliminated
            for row in 0..9 {
                for col in 0..9 {
                    let pos = (row, col);
                    if pos != start_pos 
                        && pos != end_pos
                        && self.original_puzzle[row][col] == 0
                        && self.candidates[row][col].contains(digit)
                        && self.positions_see_each_other(start_pos, pos)
                        && self.positions_see_each_other(end_pos, pos) {
                        return Some(pos);
                    }
                }
            }
        }
        
        // Look for other AIC patterns (Nice Loops, etc.)
        // For now, keep it simple with the basic contradiction pattern
        
        None
    }
    
    /// Get a copy of the current candidates for debugging/analysis.
    pub fn get_candidates(&self) -> Vec<Vec<SudokuCandidateSet>> {
        self.candidates.clone()
    }
    
    /// Solve the Sudoku puzzle and return detailed results.
    /// Note: This consumes the solver as the underlying model is consumed during solving.
    pub fn solve(mut self) -> SudokuResult {
        let start = Instant::now();
        
        // Apply advanced techniques iteratively until no more progress
        let mut technique_iterations = 0;
        while self.apply_advanced_techniques() && technique_iterations < 10 {
            technique_iterations += 1;
        }
        
        let solution = self.model.solve();
        let duration = start.elapsed();
        let duration_ms = duration.as_secs_f64() * 1000.0;
        
        match solution {
            Ok(sol) => {
                let propagations = sol.stats.propagation_count;
                let nodes = sol.stats.node_count;
                let pure_propagation = nodes == 0;
                
                // Extract solution grid
                let mut result_grid = [[0; 9]; 9];
                for row in 0..9 {
                    for col in 0..9 {
                        if let Val::ValI(value) = sol[self.grid[row][col]] {
                            result_grid[row][col] = value;
                        }
                    }
                }
                
                SudokuResult {
                    solution: Some(result_grid),
                    propagations,
                    nodes,
                    duration_ms,
                    pure_propagation,
                }
            }
            Err(_) => {
                SudokuResult {
                    solution: None,
                    propagations: 0,
                    nodes: 0,
                    duration_ms,
                    pure_propagation: false,
                }
            }
        }
    }
    
    /// Get the original puzzle that was provided to the solver.
    pub fn original_puzzle(&self) -> [[i32; 9]; 9] {
        self.original_puzzle
    }
    
    /// Count the number of clues (non-zero cells) in the original puzzle.
    pub fn clue_count(&self) -> usize {
        self.original_puzzle
            .iter()
            .flatten()
            .filter(|&&cell| cell != 0)
            .count()
    }
    
    /// Verify that a completed grid is a valid Sudoku solution.
    pub fn verify_solution(grid: &[[i32; 9]; 9]) -> bool {
        // Check all values are 1-9
        for row in grid {
            for &cell in row {
                if cell < 1 || cell > 9 {
                    return false;
                }
            }
        }
        
        // Check rows
        for row in 0..9 {
            let mut seen = [false; 10];
            for col in 0..9 {
                let val = grid[row][col] as usize;
                if seen[val] {
                    return false;
                }
                seen[val] = true;
            }
        }
        
        // Check columns
        for col in 0..9 {
            let mut seen = [false; 10];
            for row in 0..9 {
                let val = grid[row][col] as usize;
                if seen[val] {
                    return false;
                }
                seen[val] = true;
            }
        }
        
        // Check 3x3 boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut seen = [false; 10];
                for r in 0..3 {
                    for c in 0..3 {
                        let val = grid[box_row * 3 + r][box_col * 3 + c] as usize;
                        if seen[val] {
                            return false;
                        }
                        seen[val] = true;
                    }
                }
            }
        }
        
        true
    }
    
    /// Parse a string representation of a Sudoku puzzle into a 9x9 grid.
    /// 
    /// # Arguments
    /// * `puzzle_str` - An 81-character string where '0' or '.' represents empty cells
    /// 
    /// # Example
    /// ```
    /// use selen::solvers::SudokuSolver;
    /// 
    /// let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    /// let grid = SudokuSolver::parse_string(&puzzle_str).unwrap();
    /// ```
    pub fn parse_string(puzzle_str: &str) -> Result<[[i32; 9]; 9], String> {
        if puzzle_str.len() != 81 {
            return Err(format!("Expected 81 characters, got {}", puzzle_str.len()));
        }
        
        let mut grid = [[0; 9]; 9];
        for (i, ch) in puzzle_str.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            
            match ch {
                '0' | '.' => grid[row][col] = 0,
                '1'..='9' => grid[row][col] = ch.to_digit(10).unwrap() as i32,
                _ => return Err(format!("Invalid character '{}' at position {}", ch, i)),
            }
        }
        
        Ok(grid)
    }
    
    /// Format a grid as a pretty-printed string with box drawing characters.
    pub fn format_grid(title: &str, grid: &[[i32; 9]; 9]) -> String {
        let mut result = String::new();
        result.push_str(&format!("{}\n", title));
        result.push_str("┌───────┬───────┬───────┐\n");
        
        for (row_idx, row) in grid.iter().enumerate() {
            result.push('│');
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    result.push_str(" ·");
                } else {
                    result.push_str(&format!(" {}", cell));
                }
                
                if (col_idx + 1) % 3 == 0 {
                    result.push_str(" │");
                }
            }
            result.push('\n');
            
            if row_idx == 2 || row_idx == 5 {
                result.push_str("├───────┼───────┼───────┤\n");
            }
        }
        result.push_str("└───────┴───────┴───────┘");
        result
    }
}

/// Convenience function to solve a Sudoku puzzle with minimal setup.
/// 
/// # Arguments
/// * `puzzle` - A 9x9 grid where 0 represents empty cells and 1-9 are clues
/// 
/// # Returns
/// The solution grid if found, None otherwise
/// 
/// # Example
/// ```
/// use selen::solvers::solve_sudoku;
/// 
/// let puzzle = [
///     [5, 3, 0, 0, 7, 0, 0, 0, 0],
///     [6, 0, 0, 1, 9, 5, 0, 0, 0],
///     [0, 9, 8, 0, 0, 0, 0, 6, 0],
///     [8, 0, 0, 0, 6, 0, 0, 0, 3],
///     [4, 0, 0, 8, 0, 3, 0, 0, 1],
///     [7, 0, 0, 0, 2, 0, 0, 0, 6],
///     [0, 6, 0, 0, 0, 0, 2, 8, 0],
///     [0, 0, 0, 4, 1, 9, 0, 0, 5],
///     [0, 0, 0, 0, 8, 0, 0, 7, 9],
/// ];
/// 
/// if let Some(solution) = solve_sudoku(puzzle) {
///     println!("Found solution!");
/// }
/// ```
pub fn solve_sudoku(puzzle: [[i32; 9]; 9]) -> Option<[[i32; 9]; 9]> {
    let solver = SudokuSolver::new(puzzle);
    let result = solver.solve();
    result.solution
}

/// Convenience function to solve a Sudoku puzzle from a string.
/// 
/// # Arguments
/// * `puzzle_str` - An 81-character string representation of the puzzle
/// 
/// # Returns
/// The solution grid if found, None otherwise
pub fn solve_sudoku_string(puzzle_str: &str) -> Option<[[i32; 9]; 9]> {
    match SudokuSolver::parse_string(puzzle_str) {
        Ok(grid) => solve_sudoku(grid),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_easy_sudoku() {
        let puzzle = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        
        let solver = SudokuSolver::new(puzzle);
        let result = solver.solve();
        
        assert!(result.solution.is_some());
        if let Some(solution) = result.solution {
            assert!(SudokuSolver::verify_solution(&solution));
        }
    }
    
    #[test]
    fn test_string_parsing() {
        let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
        let grid = SudokuSolver::parse_string(puzzle_str).unwrap();
        
        assert_eq!(grid[0][0], 5);
        assert_eq!(grid[0][1], 3);
        assert_eq!(grid[0][2], 0);
        assert_eq!(grid[8][8], 9);
    }
    
    #[test]
    fn test_convenience_functions() {
        let puzzle_str = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
        let solution = solve_sudoku_string(puzzle_str);
        
        assert!(solution.is_some());
        if let Some(grid) = solution {
            assert!(SudokuSolver::verify_solution(&grid));
        }
    }
}
