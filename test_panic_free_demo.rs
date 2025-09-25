// Demo program showing panic-free BitSetDomain behavior
use selen::variables::domain::bitset_domain::BitSetDomain;

fn main() {
    println!("🛡️ Testing panic-free BitSetDomain implementation");
    
    // Test 1: Valid domain creation
    println!("\n1. Creating valid domain (1-9 for Sudoku):");
    let sudoku_domain = BitSetDomain::new(1, 9);
    if sudoku_domain.is_invalid() {
        println!("   ❌ Sudoku domain is invalid!");
    } else {
        println!("   ✅ Sudoku domain created successfully: size = {}", sudoku_domain.size());
    }
    
    // Test 2: Maximum allowed domain
    println!("\n2. Creating maximum allowed domain (128 values):");
    let max_domain = BitSetDomain::new(0, 127);
    if max_domain.is_invalid() {
        println!("   ❌ Max domain is invalid!");
    } else {
        println!("   ✅ Max domain created successfully: size = {}", max_domain.size());
    }
    
    // Test 3: Domain that would have caused panic before
    println!("\n3. Creating oversized domain (129 values - would panic before):");
    let oversized_domain = BitSetDomain::new(0, 128);
    if oversized_domain.is_invalid() {
        println!("   ✅ Oversized domain correctly returned as invalid (no panic!)");
        println!("   📊 Invalid domain stats: size = {}, is_empty = {}", 
                oversized_domain.size(), oversized_domain.is_empty());
    } else {
        println!("   ❌ Oversized domain should be invalid!");
    }
    
    // Test 4: Even larger domain
    println!("\n4. Creating extremely large domain (1000 values):");
    let huge_domain = BitSetDomain::new(1, 1000);
    if huge_domain.is_invalid() {
        println!("   ✅ Huge domain correctly returned as invalid (no panic!)");
    } else {
        println!("   ❌ Huge domain should be invalid!");
    }
    
    // Test 5: Domain from values that exceed limit
    println!("\n5. Creating domain from too many values:");
    let large_values: Vec<i32> = (1..200).collect(); // 199 values
    let domain_from_values = BitSetDomain::new_from_values(large_values);
    if domain_from_values.is_invalid() {
        println!("   ✅ Domain from too many values correctly returned as invalid");
    } else {
        println!("   ❌ Domain from too many values should be invalid!");
    }
    
    // Test 6: Safe operations on invalid domain
    println!("\n6. Testing operations on invalid domain:");
    let mut invalid = BitSetDomain::new(0, 200);
    assert!(invalid.is_invalid());
    println!("   - contains(50): {}", invalid.contains(50));
    println!("   - insert(75): {}", invalid.insert(75));
    println!("   - remove(100): {}", invalid.remove(100));
    println!("   - is_empty(): {}", invalid.is_empty());
    println!("   ✅ All operations on invalid domain completed safely");
    
    println!("\n🎉 All tests completed without panics! BitSetDomain is now panic-free.");
}