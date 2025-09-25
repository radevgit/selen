use selen::variables::domain::bitset_domain::BitSetDomain;

fn main() {
    println!("🔧 Testing u128 BitSetDomain Upgrade");
    println!("====================================");
    
    // Test basic domain operations
    let mut domain = BitSetDomain::new(1, 9);
    println!("Created domain 1-9: size = {}", domain.size());
    println!("Domain values: {:?}", domain.to_vec());
    
    // Test bit operations
    println!("Contains 5: {}", domain.contains(5));
    domain.remove(5);
    println!("After removing 5: size = {}, contains 5: {}", domain.size(), domain.contains(5));
    println!("Domain values: {:?}", domain.to_vec());
    
    // Test mask operations
    println!("Domain mask: 0b{:b}", domain.get_mask());
    
    // Test intersection
    let mut domain2 = BitSetDomain::new(1, 9);
    domain2.remove(1);
    domain2.remove(2);
    domain2.remove(3);
    
    println!("Domain2 values: {:?}", domain2.to_vec());
    let mut intersection = domain.clone();
    let changed = intersection.intersect_with(&domain2);
    println!("Intersection changed: {}, values: {:?}", changed, intersection.to_vec());
    
    // Test larger domains (up to 128)
    let large_domain = BitSetDomain::new(1, 100);
    println!("Large domain 1-100: size = {}", large_domain.size());
    println!("Large domain min: {:?}, max: {:?}", large_domain.min(), large_domain.max());
    
    println!("✅ BitSetDomain u128 upgrade test complete!");
}