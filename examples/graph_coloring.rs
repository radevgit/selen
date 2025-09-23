//! Graph Coloring Problem
//! 
//! Classic CSP problem: Color the vertices of a graph such that no two adjacent 
//! vertices have the same color, using the minimum number of colors possible.
//! 
//! This example demonstrates:
//! - Constraint propagation with not-equal constraints
//! - Graph representation in CSP
//! - Multiple graph topologies (planar, complete, bipartite)
//! - Performance comparison between different graph types

use selen::prelude::*;
use selen::post;
use std::collections::HashMap;

/// Represents a graph with vertices and edges
#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<usize>,
    pub edges: Vec<(usize, usize)>,
    pub name: String,
}

impl Graph {
    pub fn new(name: &str, vertices: Vec<usize>, edges: Vec<(usize, usize)>) -> Self {
        Self {
            vertices,
            edges,
            name: name.to_string(),
        }
    }

    /// Create a complete graph K_n (every vertex connected to every other)
    pub fn complete(n: usize) -> Self {
        let vertices = (0..n).collect();
        let mut edges = Vec::new();
        
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j));
            }
        }
        
        Self::new(&format!("K_{}", n), vertices, edges)
    }

    /// Create a cycle graph C_n (vertices arranged in a circle)
    pub fn cycle(n: usize) -> Self {
        let vertices = (0..n).collect();
        let mut edges = Vec::new();
        
        for i in 0..n {
            edges.push((i, (i + 1) % n));
        }
        
        Self::new(&format!("C_{}", n), vertices, edges)
    }

    /// Create a bipartite graph K_{m,n}
    pub fn bipartite(m: usize, n: usize) -> Self {
        let vertices = (0..(m + n)).collect();
        let mut edges = Vec::new();
        
        for i in 0..m {
            for j in m..(m + n) {
                edges.push((i, j));
            }
        }
        
        Self::new(&format!("K_{},{}", m, n), vertices, edges)
    }

    /// Create the Petersen graph (classic example)
    pub fn petersen() -> Self {
        let vertices = (0..10).collect();
        let edges = vec![
            // Outer pentagon
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0),
            // Inner pentagram
            (5, 7), (7, 9), (9, 6), (6, 8), (8, 5),
            // Connections between outer and inner
            (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),
        ];
        
        Self::new("Petersen", vertices, edges)
    }

    /// Create a planar graph that requires 4 colors
    pub fn planar_4_colorable() -> Self {
        let vertices = (0..8).collect();
        let edges = vec![
            (0, 1), (0, 2), (0, 3),
            (1, 2), (1, 4), (1, 5),
            (2, 3), (2, 5), (2, 6),
            (3, 6), (3, 7),
            (4, 5), (4, 7),
            (5, 6), (5, 7),
            (6, 7),
        ];
        
        Self::new("Planar4Color", vertices, edges)
    }

    pub fn get_neighbors(&self, vertex: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for &(u, v) in &self.edges {
            if u == vertex {
                neighbors.push(v);
            } else if v == vertex {
                neighbors.push(u);
            }
        }
        neighbors
    }
}

/// Solve graph coloring problem with given number of colors
pub fn solve_graph_coloring(graph: &Graph, num_colors: usize) -> Result<Solution, String> {
    let mut model = Model::default();
    
    // Create variables for each vertex (color assignment)
    let mut color_vars = HashMap::new();
    for &vertex in &graph.vertices {
        let var = model.int(0, num_colors as i32 - 1);
        color_vars.insert(vertex, var);
    }
    
    // Add constraints: adjacent vertices must have different colors
    for &(u, v) in &graph.edges {
        let var_u = color_vars[&u];
        let var_v = color_vars[&v];
        post!(model, var_u != var_v);
    }
    
    // Solve the model
    match model.solve() {
        Ok(solution) => {
            println!("✅ Successfully colored {} with {} colors!", graph.name, num_colors);
            
            // Print the coloring
            for &vertex in &graph.vertices {
                if let Val::ValI(color) = solution[color_vars[&vertex]] {
                    println!("  Vertex {}: Color {}", vertex, color);
                }
            }
            
            // Verify the solution
            verify_coloring(&graph, &solution, &color_vars)?;
            
            Ok(solution)
        },
        Err(_) => {
            Err(format!("❌ Cannot color {} with {} colors", graph.name, num_colors))
        }
    }
}

/// Find the chromatic number (minimum colors needed)
pub fn find_chromatic_number(graph: &Graph) -> usize {
    println!("🔍 Finding chromatic number for {}...", graph.name);
    
    // Binary search for minimum colors
    let mut low = 1;
    let mut high = graph.vertices.len();
    let mut chromatic_number = high;
    
    while low <= high {
        let mid = (low + high) / 2;
        
        match solve_graph_coloring(graph, mid) {
            Ok(_) => {
                chromatic_number = mid;
                high = mid - 1;
                println!("  ✓ {} colors work", mid);
            },
            Err(_) => {
                low = mid + 1;
                println!("  ✗ {} colors insufficient", mid);
            }
        }
    }
    
    println!("🎯 Chromatic number of {}: {}", graph.name, chromatic_number);
    chromatic_number
}

/// Verify that a coloring is valid
fn verify_coloring(
    graph: &Graph, 
    solution: &Solution, 
    color_vars: &HashMap<usize, VarId>
) -> Result<(), String> {
    for &(u, v) in &graph.edges {
        let color_u = if let Val::ValI(val) = solution[color_vars[&u]] { val } else { return Err("Invalid solution".to_string()); };
        let color_v = if let Val::ValI(val) = solution[color_vars[&v]] { val } else { return Err("Invalid solution".to_string()); };
        
        if color_u == color_v {
            return Err(format!("Invalid coloring: vertices {} and {} both have color {}", 
                              u, v, color_u));
        }
    }
    
    println!("  ✓ Coloring verified as valid");
    Ok(())
}

/// Analyze graph properties relevant to coloring
fn analyze_graph(graph: &Graph) {
    println!("\n📊 Graph Analysis: {}", graph.name);
    println!("  Vertices: {}", graph.vertices.len());
    println!("  Edges: {}", graph.edges.len());
    
    // Calculate maximum degree
    let max_degree = graph.vertices.iter()
        .map(|&v| graph.get_neighbors(v).len())
        .max()
        .unwrap_or(0);
    
    println!("  Maximum degree: {}", max_degree);
    println!("  Upper bound (degree + 1): {}", max_degree + 1);
    
    // Check if bipartite (would need only 2 colors)
    let is_bipartite = check_bipartite(graph);
    if is_bipartite {
        println!("  Graph is bipartite (chromatic number ≤ 2)");
    }
}

/// Simple bipartite check using 2-coloring
fn check_bipartite(graph: &Graph) -> bool {
    if graph.vertices.is_empty() {
        return true;
    }
    
    let mut model = Model::default();
    let mut color_vars = HashMap::new();
    
    // Only use 2 colors
    for &vertex in &graph.vertices {
        let var = model.int(0, 1);
        color_vars.insert(vertex, var);
    }
    
    // Add edge constraints
    for &(u, v) in &graph.edges {
        let var_u = color_vars[&u];
        let var_v = color_vars[&v];
        post!(model, var_u != var_v);
    }
    
    model.solve().is_ok()
}

fn main() {
    println!("🎨 Graph Coloring Problems\n");
    
    // Test various graph types
    let graphs = vec![
        Graph::cycle(5),           // Should need 3 colors (odd cycle)
        Graph::cycle(6),           // Should need 2 colors (even cycle)
        Graph::complete(4),        // Should need 4 colors
        Graph::bipartite(3, 4),    // Should need 2 colors
        Graph::petersen(),         // Should need 3 colors
        Graph::planar_4_colorable(), // Should need 4 colors
    ];
    
    for graph in graphs {
        analyze_graph(&graph);
        let chromatic_number = find_chromatic_number(&graph);
        
        // Show the actual coloring
        println!("\n🎨 Optimal coloring:");
        solve_graph_coloring(&graph, chromatic_number).unwrap();
        
        println!("\n{}\n", "=".repeat(50));
    }
    
    // Performance test with larger graphs
    println!("⚡ Performance Test: Complete Graphs");
    for n in 3..=6 {
        let graph = Graph::complete(n);
        let start = std::time::Instant::now();
        let chromatic_number = find_chromatic_number(&graph);
        let duration = start.elapsed();
        
        println!("K_{}: {} colors, solved in {:?}", n, chromatic_number, duration);
        assert_eq!(chromatic_number, n, "Complete graph K_{} should need {} colors", n, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_graphs() {
        // Odd cycles need 3 colors
        let c5 = Graph::cycle(5);
        assert_eq!(find_chromatic_number(&c5), 3);
        
        // Even cycles need 2 colors
        let c6 = Graph::cycle(6);
        assert_eq!(find_chromatic_number(&c6), 2);
    }
    
    #[test]
    fn test_complete_graphs() {
        // Complete graphs need n colors
        for n in 2..=5 {
            let kn = Graph::complete(n);
            assert_eq!(find_chromatic_number(&kn), n);
        }
    }
    
    #[test]
    fn test_bipartite_graphs() {
        // Bipartite graphs need at most 2 colors
        let k33 = Graph::bipartite(3, 3);
        assert_eq!(find_chromatic_number(&k33), 2);
        
        let k24 = Graph::bipartite(2, 4);
        assert_eq!(find_chromatic_number(&k24), 2);
    }
    
    #[test]
    fn test_petersen_graph() {
        // Petersen graph is 3-colorable
        let petersen = Graph::petersen();
        assert_eq!(find_chromatic_number(&petersen), 3);
    }
    
    #[test]
    fn test_bipartite_detection() {
        assert!(check_bipartite(&Graph::bipartite(3, 4)));
        assert!(check_bipartite(&Graph::cycle(6))); // Even cycle
        assert!(!check_bipartite(&Graph::cycle(5))); // Odd cycle
        assert!(!check_bipartite(&Graph::complete(3))); // Triangle
    }
}