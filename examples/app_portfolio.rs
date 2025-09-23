//! Portfolio Balance Calculator
//!
//! A financial portfolio calculator demonstrating floating-point
//! constraint programming. This example shows:
//! - Float variables for portfolio weights and returns  
//! - Percentage-based constraints 
//! - Simple constraint satisfaction with floating-point calculations

use selen::prelude::*;
use selen::{post};

fn main() {
    println!("💰 Portfolio Balance Calculator");
    println!("===============================");
    
    // Create a model for simple portfolio balancing
    let mut m = Model::default();
    
    // Investment options with expected annual returns (as percentages)
    let assets = ["Stocks", "Bonds"];
    let expected_returns = [float(8.5), float(3.2)]; // % per year
    let risk_levels = [float(15.0), float(2.0)]; // volatility %
    
    println!("Available Investment Assets:");
    for (i, asset) in assets.iter().enumerate() {
        let ret = match expected_returns[i] { 
            Val::ValF(r) => r, 
            Val::ValI(r) => r as f64 
        };
        let risk = match risk_levels[i] { 
            Val::ValF(r) => r, 
            Val::ValI(r) => r as f64 
        };
        println!("  {}: {:.1}% return, {:.1}% risk", asset, ret, risk);
    }
    
    // Portfolio allocation: simple two-asset model
    // Stocks: 30% to 70% (balanced range)
    let stock_weight = m.float(30.0, 70.0);
    
    // Bonds: 30% to 70% (balanced range)
    let bond_weight = m.float(30.0, 70.0);
    
    // Constraint: Total allocation must be 100%
    let total_weight = m.add(stock_weight, bond_weight);
    post!(m, total_weight == float(100.0));
    
    println!("\nConstraints:");
    println!("  • Stock allocation: 30% to 70%");
    println!("  • Bond allocation: 30% to 70%");
    println!("  • Total allocation: Must equal 100%");
    
    // Find a feasible solution
    println!("\n🎯 Finding portfolio allocation...");
    
    let solution = match m.solve() {
        Ok(sol) => sol,
        Err(err) => {
            println!("❌ No feasible solution found: {}", err);
            return;
        }
    };
    
    println!("\n📊 Portfolio Allocation Results:");
    println!("=================================");
    
    let stock_allocation = match solution[stock_weight] {
        Val::ValF(w) => w,
        Val::ValI(w) => w as f64,
    };
    
    let bond_allocation = match solution[bond_weight] {
        Val::ValF(w) => w,
        Val::ValI(w) => w as f64,
    };
    
    println!("  Stocks: {:.1}% allocation", stock_allocation);
    println!("  Bonds: {:.1}% allocation", bond_allocation);
    println!("  Total: {:.1}%", stock_allocation + bond_allocation);
    
    // Calculate portfolio metrics using floating-point precision
    let stock_return_contrib = (stock_allocation / 100.0) * 8.5;
    let bond_return_contrib = (bond_allocation / 100.0) * 3.2;
    let total_expected_return = stock_return_contrib + bond_return_contrib;
    
    let stock_risk_contrib = (stock_allocation / 100.0) * 15.0;
    let bond_risk_contrib = (bond_allocation / 100.0) * 2.0;
    let portfolio_risk = stock_risk_contrib + bond_risk_contrib;
    
    println!("\n💼 Portfolio Summary:");
    println!("  Expected annual return: {:.2}%", total_expected_return);
    println!("  Estimated portfolio risk: {:.2}%", portfolio_risk);
    println!("  Risk-adjusted return ratio: {:.3}", total_expected_return / portfolio_risk);
    
    println!("\n🔍 Investment Profile:");
    if stock_allocation > 60.0 {
        println!("  • Growth-oriented strategy (higher stock allocation)");
    } else if stock_allocation > 45.0 {
        println!("  • Balanced approach (moderate stock allocation)");
    } else {
        println!("  • Conservative strategy (lower stock allocation)");
    }
    
    println!("\n✅ Portfolio calculation complete!");
    println!("   This demonstrates floating-point constraint programming");
    println!("   with precise percentage calculations.");
    
    println!("\n🔢 Float() Function Demonstration:");
    println!("   • Asset returns: float(8.5), float(3.2)");
    println!("   • Risk levels: float(15.0), float(2.0)");
    println!("   • Stock weight range: float(30.0) to float(70.0)");
    println!("   • Portfolio total: float(100.0)");
    println!("   → All calculations maintain floating-point precision!");
}
