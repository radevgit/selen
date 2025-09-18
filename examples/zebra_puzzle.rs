//! Zebra Puzzle - Einstein's Logic Puzzle
//! 
//! Who owns the zebra? Logical deduction puzzle with multiple constraints.

use cspsolver::prelude::*;

fn main() {
    println!("🦓 Einstein's Zebra Puzzle (Simplified)");
    println!("========================================");
    
    let mut model = Model::default();
    
    // Each attribute has 5 values for 5 houses (positions 1-5)
    // Colors: Red=1, Green=2, Blue=3, Yellow=4, White=5
    let red_house = model.int(1, 5);
    let green_house = model.int(1, 5);
    let blue_house = model.int(1, 5);
    let yellow_house = model.int(1, 5);
    let white_house = model.int(1, 5);
    
    // Pets: Dog=1, Cat=2, Bird=3, Fish=4, Zebra=5
    let dog = model.int(1, 5);
    let cat = model.int(1, 5);
    let bird = model.int(1, 5);
    let fish = model.int(1, 5);
    let zebra = model.int(1, 5);
    
    // Drinks: Tea=1, Coffee=2, Water=3, Beer=4, Milk=5
    let tea = model.int(1, 5);
    let coffee = model.int(1, 5);
    let water = model.int(1, 5);
    let beer = model.int(1, 5);
    let milk = model.int(1, 5);
    
    // Cigarettes: Kools=1, Chesterfield=2, Winston=3, Lucky=4, Parliament=5
    let kools = model.int(1, 5);
    let chesterfield = model.int(1, 5);
    let winston = model.int(1, 5);
    let lucky = model.int(1, 5);
    let parliament = model.int(1, 5);
    
    // Nationalities: British=1, Swedish=2, Danish=3, Norwegian=4, German=5
    let british = model.int(1, 5);
    let swedish = model.int(1, 5);
    let danish = model.int(1, 5);
    let norwegian = model.int(1, 5);
    let german = model.int(1, 5);
    
    // All values in each category must be different
    post!(model, alldiff(vec![red_house, green_house, blue_house, yellow_house, white_house]));
    post!(model, alldiff(vec![dog, cat, bird, fish, zebra]));
    post!(model, alldiff(vec![tea, coffee, water, beer, milk]));
    post!(model, alldiff(vec![kools, chesterfield, winston, lucky, parliament]));
    post!(model, alldiff(vec![british, swedish, danish, norwegian, german]));
    
    // Basic constraints (simplified version of Einstein's puzzle)
    // 1. The British person lives in the red house
    post!(model, british == red_house);
    
    // 2. The Swedish person keeps dogs
    post!(model, swedish == dog);
    
    // 3. The Danish person drinks tea
    post!(model, danish == tea);
    
    // 4. The green house owner drinks coffee
    post!(model, green_house == coffee);
    
    // 5. The person who smokes Parliament rears birds
    post!(model, parliament == bird);
    
    // 6. The owner of the yellow house smokes Kools
    post!(model, yellow_house == kools);
    
    // 7. The man living in the center house drinks milk
    post!(model, milk == int(3));
    
    // 8. The Norwegian lives in the first house
    post!(model, norwegian == int(1));
    
    println!("🔍 Solving the puzzle...");
    match model.solve() {
        Ok(solution) => {
            println!("✅ Solution found!");
            
            // Extract zebra position
            if let Val::ValI(zebra_house) = solution[zebra] {
                println!("\n🦓 The zebra is owned by the person in house {}!", zebra_house);
                
                // Show which nationality owns the zebra
                let nationalities = [
                    ("British", solution[british]),
                    ("Swedish", solution[swedish]),
                    ("Danish", solution[danish]),
                    ("Norwegian", solution[norwegian]),
                    ("German", solution[german]),
                ];
                
                for (name, val) in &nationalities {
                    if let Val::ValI(pos) = val {
                        if *pos == zebra_house {
                            println!("🏆 The {} person owns the zebra!", name);
                        }
                    }
                }
            }
        },
        Err(_) => {
            println!("❌ No solution found with current constraints!");
        }
    }
}
