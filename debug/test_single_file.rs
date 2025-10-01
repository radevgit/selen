use selen::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <flatzinc_file>", args[0]);
        std::process::exit(1);
    }
    
    let filepath = Path::new(&args[1]);
    let mut model = Model::default();
    
    match model.from_flatzinc_file(&filepath) {
        Ok(_) => {
            println!("✓ Successfully loaded {}", filepath.display());
        }
        Err(e) => {
            eprintln!("✗ Error loading {}: {}", filepath.display(), e);
            std::process::exit(1);
        }
    }
}
