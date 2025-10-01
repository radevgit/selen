use selen::prelude::*;
use std::path::Path;

#[test]
fn find_panic_file() {
    let examples_dir = Path::new("src/zinc/flatzinc");
    
    // Test files with unbounded domains
    let test_files = vec![
        "euler_30.fzn",
        "evens.fzn",
    ];
    
    for file in test_files {
        let path = examples_dir.join(file);
        if !path.exists() {
            println!("SKIP: {}", file);
            continue;
        }
        
        print!("Testing {}: ", file);
        let mut model = Model::default();
        match model.from_flatzinc_file(&path) {
            Ok(_) => println!("OK"),
            Err(e) => println!("ERROR: {}", e),
        }
    }
}
