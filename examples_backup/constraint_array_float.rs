use selen::prelude::*;

fn main() {
    println!("=== Array Float Constraints Examples ===\n");
    
    // Example 1: array_float_minimum
    println!("📝 Example 1: array_float_minimum - Find Minimum Temperature");
    {
        let mut model = Model::default();
        
        // Temperature readings from different sensors
        let sensor1 = model.float(18.5, 18.5);
        let sensor2 = model.float(21.3, 21.3);
        let sensor3 = model.float(19.7, 19.7);
        let sensor4 = model.float(17.2, 17.2);
        
        let sensors = vec![sensor1, sensor2, sensor3, sensor4];
        let min_temp = model.array_float_minimum(&sensors)
            .expect("Should find minimum");
        
        match model.solve() {
            Ok(solution) => {
                let min_val = solution.get_float(min_temp);
                println!("  Temperature readings:");
                for (i, &sensor) in sensors.iter().enumerate() {
                    println!("    Sensor {}: {:.1}°C", i + 1, solution.get_float(sensor));
                }
                println!("  Minimum temperature: {:.1}°C", min_val);
                assert!((min_val - 17.2).abs() < 1e-9);
                println!("  ✓ Correctly identified minimum");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 2: array_float_maximum
    println!("📝 Example 2: array_float_maximum - Find Maximum Score");
    {
        let mut model = Model::default();
        
        // Test scores for different students
        let scores = vec![
            model.float(87.5, 87.5),
            model.float(92.3, 92.3),
            model.float(78.9, 78.9),
            model.float(95.1, 95.1),
            model.float(88.7, 88.7),
        ];
        
        let max_score = model.array_float_maximum(&scores)
            .expect("Should find maximum");
        
        match model.solve() {
            Ok(solution) => {
                let max_val = solution.get_float(max_score);
                println!("  Test scores: {:?}", 
                         scores.iter().map(|&s| solution.get_float(s)).collect::<Vec<_>>());
                println!("  Highest score: {:.1}", max_val);
                assert!((max_val - 95.1).abs() < 1e-9);
                println!("  ✓ Correctly identified maximum");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 3: array_float_element with fixed index
    println!("📝 Example 3: array_float_element - Select Price by Index");
    {
        let mut model = Model::default();
        
        // Product prices
        let prices = vec![
            model.float(19.99, 19.99),  // Product 0
            model.float(24.50, 24.50),  // Product 1
            model.float(15.75, 15.75),  // Product 2
            model.float(32.00, 32.00),  // Product 3
        ];
        
        let selected_index = model.int(2, 2); // Select product 2
        let selected_price = model.float(0.0, 50.0);
        
        model.array_float_element(selected_index, &prices, selected_price);
        
        match model.solve() {
            Ok(solution) => {
                let idx = solution.get_int(selected_index);
                let price = solution.get_float(selected_price);
                println!("  Selected product: {}", idx);
                println!("  Price: ${:.2}", price);
                assert_eq!(idx, 2);
                assert!((price - 15.75).abs() < 1e-9);
                println!("  ✓ Correctly selected price");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 4: array_float_element with variable index
    println!("📝 Example 4: Variable Index Selection");
    {
        let mut model = Model::default();
        
        // Array of values
        let values = vec![
            model.float(5.5, 5.5),
            model.float(10.2, 10.2),
            model.float(15.7, 15.7),
            model.float(20.1, 20.1),
        ];
        
        let index = model.int(0, 3); // Can select any index
        let result = model.float(0.0, 30.0);
        
        model.array_float_element(index, &values, result);
        
        // Constraint: we want the result to be the value 15.7
        model.props.equals(result, Val::ValF(15.7));
        
        match model.solve() {
            Ok(solution) => {
                let idx = solution.get_int(index);
                let res = solution.get_float(result);
                println!("  Constraint: result must equal 15.7");
                println!("  Selected index: {}", idx);
                println!("  Result value: {:.1}", res);
                assert_eq!(idx, 2, "Should select index 2");
                assert!((res - 15.7).abs() < 1e-9);
                println!("  ✓ Constraint solver found correct index");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 5: Combined min, max, and element
    println!("📝 Example 5: Statistical Analysis");
    {
        let mut model = Model::default();
        
        // Dataset
        let data = vec![
            model.float(12.5, 12.5),
            model.float(18.3, 18.3),
            model.float(9.7, 9.7),
            model.float(22.1, 22.1),
            model.float(15.8, 15.8),
        ];
        
        let min_val = model.array_float_minimum(&data)
            .expect("Should find min");
        let max_val = model.array_float_maximum(&data)
            .expect("Should find max");
        
        // Find which index has the maximum value
        let max_index = model.int(0, 4);
        let max_element = model.float(0.0, 30.0);
        model.array_float_element(max_index, &data, max_element);
        model.props.equals(max_element, max_val);
        
        match model.solve() {
            Ok(solution) => {
                let min = solution.get_float(min_val);
                let max = solution.get_float(max_val);
                let max_idx = solution.get_int(max_index);
                
                println!("  Dataset: {:?}", 
                         data.iter().map(|&d| solution.get_float(d)).collect::<Vec<_>>());
                println!("  Minimum: {:.1}", min);
                println!("  Maximum: {:.1}", max);
                println!("  Maximum is at index: {}", max_idx);
                println!("  Range: {:.1}", max - min);
                
                assert!((min - 9.7).abs() < 1e-9);
                assert!((max - 22.1).abs() < 1e-9);
                assert_eq!(max_idx, 3);
                println!("  ✓ Statistical analysis complete");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 6: Portfolio Optimization
    println!("📝 Example 6: Investment Portfolio Selection");
    {
        let mut model = Model::default();
        
        // Expected returns for different investments
        let returns = vec![
            model.float(5.2, 5.2),   // Investment A: 5.2%
            model.float(7.8, 7.8),   // Investment B: 7.8%
            model.float(4.5, 4.5),   // Investment C: 4.5%
            model.float(9.1, 9.1),   // Investment D: 9.1%
            model.float(6.3, 6.3),   // Investment E: 6.3%
        ];
        
        let best_return = model.array_float_maximum(&returns)
            .expect("Should find best return");
        let worst_return = model.array_float_minimum(&returns)
            .expect("Should find worst return");
        
        match model.solve() {
            Ok(solution) => {
                let best = solution.get_float(best_return);
                let worst = solution.get_float(worst_return);
                
                println!("  Investment returns:");
                for (i, &ret) in returns.iter().enumerate() {
                    let r = solution.get_float(ret);
                    let mark = if (r - best).abs() < 1e-9 { " ⭐ BEST" } 
                              else if (r - worst).abs() < 1e-9 { " ⚠ WORST" }
                              else { "" };
                    println!("    Investment {}: {:.1}%{}", (b'A' + i as u8) as char, r, mark);
                }
                println!("  Best return: {:.1}%", best);
                println!("  Worst return: {:.1}%", worst);
                println!("  Spread: {:.1}%", best - worst);
                
                assert!((best - 9.1).abs() < 1e-9);
                assert!((worst - 4.5).abs() < 1e-9);
                println!("  ✓ Portfolio analysis complete");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 7: Dynamic Pricing
    println!("📝 Example 7: Dynamic Price Selection");
    {
        let mut model = Model::default();
        
        // Prices based on demand level (0=low, 1=medium, 2=high, 3=peak)
        let price_tiers = vec![
            model.float(9.99, 9.99),    // Low demand
            model.float(12.99, 12.99),  // Medium demand
            model.float(15.99, 15.99),  // High demand
            model.float(19.99, 19.99),  // Peak demand
        ];
        
        let demand_level = model.int(0, 3);
        let current_price = model.float(0.0, 25.0);
        
        model.array_float_element(demand_level, &price_tiers, current_price);
        
        // Business rule: price should be at least $12
        model.props.greater_than_or_equals(current_price, Val::ValF(12.0));
        
        match model.solve() {
            Ok(solution) => {
                let level = solution.get_int(demand_level);
                let price = solution.get_float(current_price);
                
                let level_name = ["Low", "Medium", "High", "Peak"][level as usize];
                println!("  Price tiers: $9.99, $12.99, $15.99, $19.99");
                println!("  Constraint: Price must be ≥ $12.00");
                println!("  Selected demand level: {} ({})", level, level_name);
                println!("  Current price: ${:.2}", price);
                
                assert!(level >= 1, "Should select medium or higher demand");
                assert!(price >= 12.0, "Price should be at least $12");
                println!("  ✓ Dynamic pricing configured");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!("\n✅ All array float constraint examples completed successfully!");
}
