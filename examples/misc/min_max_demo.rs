/// Demonstration of the new vector-based Min/Max constraints
/// 
/// This example shows how to use the newly implemented min() and max() constraints
/// to solve practical optimization problems.

use cspsolver::prelude::*;

fn main() {
    println!("🎯 CSP Solver - Min/Max Constraints Demonstration\n");

    // Example 1: Basic Min/Max Usage
    println!("Example 1: Basic Min/Max with Multiple Variables");
    {
        let mut m = Model::default();
        
        let a = m.int(1, 10);
        let b = m.int(5, 15);
        let c = m.int(3, 8);
        
        let minimum = m.min(&[a, b, c]).expect("non-empty variable list");
        let maximum = m.max(&[a, b, c]).expect("non-empty variable list");
        
        // Add some constraints
        post!(m, minimum == 4);  // min must be 4
        post!(m, maximum <= 12); // max must be <= 12
        
        if let Ok(solution) = m.solve() {
            println!("  Variables: a={:?}, b={:?}, c={:?}", 
                     solution[a], solution[b], solution[c]);
            println!("  Minimum: {:?}, Maximum: {:?}", 
                     solution[minimum], solution[maximum]);
        }
    }
    
    println!();

    // Example 2: Resource Allocation Problem
    println!("Example 2: Resource Allocation (Finding Bottleneck)");
    {
        let mut m = Model::default();
        
        // Represent resource capacities for different departments
        let engineering = m.int(50, 100);  // 50-100 engineers
        let marketing = m.int(20, 80);     // 20-80 marketing people  
        let sales = m.int(30, 90);         // 30-90 sales people
        let support = m.int(10, 40);       // 10-40 support staff
        
        // Find the department with minimum resources (bottleneck)
        let bottleneck = m.min(&[engineering, marketing, sales, support]);
        
        // Find the department with maximum resources
        let largest_dept = m.max(&[engineering, marketing, sales, support]).expect("non-empty variable list");
        
        // Constraint: bottleneck should be at least 25
        post!(m, bottleneck >= 25);
        
        // Constraint: largest department should be at most 85
        post!(m, largest_dept <= 85);
        
        // Constraint: engineering must have at least 60 people
        post!(m, engineering >= 60);
        
        if let Ok(solution) = m.solve() {
            println!("  Engineering: {:?} people", solution[engineering]);
            println!("  Marketing:   {:?} people", solution[marketing]);
            println!("  Sales:       {:?} people", solution[sales]);
            println!("  Support:     {:?} people", solution[support]);
            println!("  Bottleneck (min): {:?} people", solution[bottleneck]);
            println!("  Largest dept (max): {:?} people", solution[largest_dept]);
        }
    }
    
    println!();

    // Example 3: Performance Metrics
    println!("Example 3: Performance Metrics Optimization");
    {
        let mut m = Model::default();
        
        // Performance scores for different systems (0-100 scale)
        let latency_score = m.int(60, 95);     // Lower latency = higher score
        let throughput_score = m.int(70, 90);  // Higher throughput = higher score
        let reliability_score = m.int(80, 98); // Higher reliability = higher score
        let cost_score = m.int(40, 85);        // Lower cost = higher score
        
        let scores = [latency_score, throughput_score, reliability_score, cost_score];
        
        // Overall system performance is limited by the weakest metric
        let overall_performance = m.min(&scores);
        
        // We want to maximize the minimum performance (improve the bottleneck)
        post!(m, overall_performance >= 75);
        
        // Best case scenario - what's the highest we can achieve?
        let best_metric = m.max(&scores).expect("non-empty variable list");
        
        // Constraint: total "effort" is limited (trade-offs between metrics)
        let total_effort = m.sum(&scores);
        post!(m, total_effort <= 320); // Limited total effort
        
        if let Ok(solution) = m.solve() {
            println!("  Latency Score:     {:?}/100", solution[latency_score]);
            println!("  Throughput Score:  {:?}/100", solution[throughput_score]);
            println!("  Reliability Score: {:?}/100", solution[reliability_score]);
            println!("  Cost Score:        {:?}/100", solution[cost_score]);
            println!("  Overall Performance (min): {:?}/100", solution[overall_performance]);
            println!("  Best Metric (max): {:?}/100", solution[best_metric]);
            println!("  Total Effort: {:?}/400", solution[total_effort]);
        }
    }
    
    println!();

    // Example 4: Mixed Integer/Float Constraints
    println!("Example 4: Mixed Integer/Float Temperature Control");
    {
        let mut m = Model::default();
        
        // Temperature readings from different sensors (in Celsius)
        let sensor1 = m.float(18.5, 25.5);  // Room temperature
        let sensor2 = m.int(20, 28);        // Thermostat (integer)
        let sensor3 = m.float(19.0, 26.0);  // Outside sensor
        
        let sensors = [sensor1, sensor2, sensor3];
        
        // Find minimum and maximum temperatures
        let min_temp = m.min(&sensors);
        let max_temp = m.max(&sensors).expect("non-empty sensors list");
        
        // Constraint: temperature range should not exceed 5 degrees
        let temp_range = m.sub(max_temp, min_temp);
        post!(m, temp_range <= 5.0);
        
        // Constraint: minimum temperature should be at least 20°C
        post!(m, min_temp >= 20.0);
        
        if let Ok(solution) = m.solve() {
            println!("  Sensor 1: {:?}°C", solution[sensor1]);
            println!("  Sensor 2: {:?}°C", solution[sensor2]);
            println!("  Sensor 3: {:?}°C", solution[sensor3]);
            println!("  Min Temperature: {:?}°C", solution[min_temp]);
            println!("  Max Temperature: {:?}°C", solution[max_temp]);
            println!("  Temperature Range: {:?}°C", solution[temp_range]);
        }
    }
    
    println!("\n✅ Min/Max constraints successfully demonstrated!");
    println!("   Features: Vector-based operations, mixed types, complex propagation");
}
