//! Zebra Puzzle Solver (New API)
//!
//! The classic Einstein's Riddle / Zebra Puzzle.
//! This example demonstrates the new unified constraint API.
//!
//! Demonstrates:
//! - m.int() for variable creation
//! - m.alldiff() for all-different constraints
//! - m.new(var.eq(other)) for equality constraints (runtime API)
//! - add(), sub(), abs() for arithmetic expressions
//! - Explicit int() constants

use selen::prelude::*;

fn main() {
    println!("🦓 Zebra Puzzle Solver (New API)");
    println!("===============================\n");
    println!("Who owns the zebra? Who drinks water?\n");

    let mut m = Model::default();

    // Each attribute (color, nationality, drink, smoke, pet) has 5 houses (1-5)
    // Colors
    let red = m.int(1, 5);
    let green = m.int(1, 5);
    let ivory = m.int(1, 5);
    let yellow = m.int(1, 5);
    let blue = m.int(1, 5);
    m.alldiff(&[red, green, ivory, yellow, blue]);

    // Nationalities
    let english = m.int(1, 5);
    let spanish = m.int(1, 5);
    let ukrainian = m.int(1, 5);
    let norwegian = m.int(1, 5);
    let japanese = m.int(1, 5);
    m.alldiff(&[english, spanish, ukrainian, norwegian, japanese]);

    // Drinks
    let coffee = m.int(1, 5);
    let tea = m.int(1, 5);
    let milk = m.int(1, 5);
    let orange_juice = m.int(1, 5);
    let water = m.int(1, 5);
    m.alldiff(&[coffee, tea, milk, orange_juice, water]);

    // Smokes
    let old_gold = m.int(1, 5);
    let kools = m.int(1, 5);
    let chesterfields = m.int(1, 5);
    let lucky_strike = m.int(1, 5);
    let parliaments = m.int(1, 5);
    m.alldiff(&[old_gold, kools, chesterfields, lucky_strike, parliaments]);

    // Pets
    let dog = m.int(1, 5);
    let snails = m.int(1, 5);
    let fox = m.int(1, 5);
    let horse = m.int(1, 5);
    let zebra = m.int(1, 5);
    m.alldiff(&[dog, snails, fox, horse, zebra]);

    // Constraints from the puzzle
    // 1. The Englishman lives in the red house
    m.new(english.eq(red));

    // 2. The Spaniard owns the dog
    m.new(spanish.eq(dog));

    // 3. Coffee is drunk in the green house
    m.new(coffee.eq(green));

    // 4. The Ukrainian drinks tea
    m.new(ukrainian.eq(tea));

    // 5. The green house is immediately to the right of the ivory house
    m.new(sub(green, ivory).eq(int(1)));

    // 6. The Old Gold smoker owns snails
    m.new(old_gold.eq(snails));

    // 7. Kools are smoked in the yellow house
    m.new(kools.eq(yellow));

    // 8. Milk is drunk in the middle house
    m.new(milk.eq(int(3)));

    // 9. The Norwegian lives in the first house
    m.new(norwegian.eq(int(1)));

    // 10. The man who smokes Chesterfields lives next to the man with the fox
    // |chesterfields - fox| = 1
    let abs_diff1 = abs(&mut m, sub(chesterfields, fox));
    m.new(abs_diff1.eq(int(1)));

    // 11. Kools are smoked in the house next to the house with the horse
    // |kools - horse| = 1
    let abs_diff2 = abs(&mut m, sub(kools, horse));
    m.new(abs_diff2.eq(int(1)));

    // 12. The Lucky Strike smoker drinks orange juice
    m.new(lucky_strike.eq(orange_juice));

    // 13. The Japanese smokes Parliaments
    m.new(japanese.eq(parliaments));

    // 14. The Norwegian lives next to the blue house
    // |norwegian - blue| = 1
    let abs_diff3 = abs(&mut m, sub(norwegian, blue));
    m.new(abs_diff3.eq(int(1)));

    println!("🔍 Solving...");
    match m.solve() {
        Ok(solution) => {
            println!("✅ Solution found!\n");

            let zebra_house = solution.get_int(zebra);
            let water_house = solution.get_int(water);

            // Find who owns the zebra
            let zebra_owner = if solution.get_int(english) == zebra_house {
                "English"
            } else if solution.get_int(spanish) == zebra_house {
                "Spanish"
            } else if solution.get_int(ukrainian) == zebra_house {
                "Ukrainian"
            } else if solution.get_int(norwegian) == zebra_house {
                "Norwegian"
            } else {
                "Japanese"
            };

            // Find who drinks water
            let water_drinker = if solution.get_int(english) == water_house {
                "English"
            } else if solution.get_int(spanish) == water_house {
                "Spanish"
            } else if solution.get_int(ukrainian) == water_house {
                "Ukrainian"
            } else if solution.get_int(norwegian) == water_house {
                "Norwegian"
            } else {
                "Japanese"
            };

            println!("🦓 The {} owns the zebra (house {})", zebra_owner, zebra_house);
            println!("💧 The {} drinks water (house {})", water_drinker, water_house);
        }
        Err(_) => {
            println!("❌ No solution found!");
        }
    }
}
