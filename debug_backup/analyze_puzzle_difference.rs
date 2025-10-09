//! Analyze the structural differences between EXTREME and PLATINUM puzzles

fn main() {
    let extreme_puzzle = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];
    
    let platinum_puzzle = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];
    
    println!("Structural Analysis:");
    
    // Count clues
    let extreme_clues = extreme_puzzle.iter().flatten().filter(|&&x| x != 0).count();
    let platinum_clues = platinum_puzzle.iter().flatten().filter(|&&x| x != 0).count();
    
    println!("Extreme clues: {}", extreme_clues);
    println!("Platinum clues: {}", platinum_clues);
    
    // Check distribution patterns
    println!("\nClue distribution by row:");
    for (i, (ext_row, plat_row)) in extreme_puzzle.iter().zip(platinum_puzzle.iter()).enumerate() {
        let ext_count = ext_row.iter().filter(|&&x| x != 0).count();
        let plat_count = plat_row.iter().filter(|&&x| x != 0).count();
        println!("Row {}: Extreme={}, Platinum={}", i, ext_count, plat_count);
    }
    
    // Check if first row being completely empty in Platinum matters
    println!("\nFirst row analysis:");
    println!("Extreme first row: {:?}", extreme_puzzle[0]);
    println!("Platinum first row: {:?}", platinum_puzzle[0]);
    
    // Check for patterns that might trigger different algorithm paths
    let extreme_zeros = extreme_puzzle.iter().flatten().filter(|&&x| x == 0).count();
    let platinum_zeros = platinum_puzzle.iter().flatten().filter(|&&x| x == 0).count();
    
    println!("\nEmpty cells:");
    println!("Extreme: {} empty cells", extreme_zeros);
    println!("Platinum: {} empty cells", platinum_zeros);
    
    // The real issue might be in the first constraint propagation
    println!("\nKey difference: Platinum has completely empty first row!");
    println!("This might trigger different constraint propagation patterns.");
}
