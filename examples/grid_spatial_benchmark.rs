//! Benchmark comparison: SpatialHash vs GridSpatialHash
//! 
//! Demonstrates performance improvement from grid-based spatial partitioning.

use std::time::Instant;
use falling_sand::{GridSpatialHash, SpatialHash};

/// Simple pseudo-random number generator for reproducibility
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next(&mut self) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state % 1000) as usize
    }
}

/// Benchmark result for a single test
struct BenchmarkResult {
    particles: usize,
    hash_build_ms: f64,
    grid_build_ms: f64,
    hash_query_ms: f64,
    grid_query_ms: f64,
}

impl BenchmarkResult {
    fn speedup_build(&self) -> f64 {
        if self.grid_build_ms > 0.0 {
            self.hash_build_ms / self.grid_build_ms
        } else { 1.0 }
    }
    
    fn speedup_query(&self) -> f64 {
        if self.grid_query_ms > 0.0 {
            self.hash_query_ms / self.grid_query_ms
        } else { 1.0 }
    }
    
    fn summary(&self) -> String {
        format!(
            "{:>6} particles | Hash: {:>7.3}ms build, {:>7.3}ms query | Grid: {:>7.3}ms build, {:>7.3}ms query | Build speedup: {:.2}x | Query speedup: {:.2}x",
            self.particles,
            self.hash_build_ms,
            self.hash_query_ms,
            self.grid_build_ms,
            self.grid_query_ms,
            self.speedup_build(),
            self.speedup_query()
        )
    }
}

/// Run comparison benchmark
fn run_comparison_benchmark(width: usize, height: usize, particles: usize, iterations: usize) -> BenchmarkResult {
    let cell_size = 16;
    
    // Generate random positions (deterministic)
    let mut rng = SimpleRng::new(particles as u64);
    let positions: Vec<(usize, usize)> = (0..particles)
        .map(|_| (rng.next() % width, rng.next() % height))
        .collect();
    
    // Sample positions for query benchmark (10%)
    let sample_size = (positions.len() / 10).max(1);
    let sample_positions: Vec<(usize, usize)> = positions.iter().take(sample_size).cloned().collect();
    
    // Benchmark SpatialHash
    let mut hash_build_times = Vec::with_capacity(iterations);
    let mut hash_query_times = Vec::with_capacity(iterations);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let mut hash = SpatialHash::new(width, height, cell_size);
        for (x, y) in &positions {
            hash.insert(*x, *y);
        }
        let build_time = start.elapsed().as_secs_f64() * 1000.0;
        hash_build_times.push(build_time);
        
        let start = Instant::now();
        let mut total_neighbors = 0;
        for (x, y) in &sample_positions {
            total_neighbors += hash.get_neighbors(*x, *y).len();
        }
        let query_time = start.elapsed().as_secs_f64() * 1000.0;
        hash_query_times.push(query_time);
        
        let _ = total_neighbors; // Use the result
    }
    
    // Benchmark GridSpatialHash
    let mut grid_build_times = Vec::with_capacity(iterations);
    let mut grid_query_times = Vec::with_capacity(iterations);
    
    for _ in 0..iterations {
        let start = Instant::now();
        let mut grid_hash = GridSpatialHash::new(width, height, cell_size);
        for (x, y) in &positions {
            grid_hash.insert(*x, *y);
        }
        let build_time = start.elapsed().as_secs_f64() * 1000.0;
        grid_build_times.push(build_time);
        
        let start = Instant::now();
        let mut total_neighbors = 0;
        for (x, y) in &sample_positions {
            total_neighbors += grid_hash.get_neighbors(*x, *y).len();
        }
        let query_time = start.elapsed().as_secs_f64() * 1000.0;
        grid_query_times.push(query_time);
        
        let _ = total_neighbors; // Use the result
    }
    
    // Average results
    let avg = |v: &Vec<f64>| v.iter().sum::<f64>() / v.len() as f64;
    
    BenchmarkResult {
        particles,
        hash_build_ms: avg(&hash_build_times),
        grid_build_ms: avg(&grid_build_times),
        hash_query_ms: avg(&hash_query_times),
        grid_query_ms: avg(&grid_query_times),
    }
}

fn main() {
    println!("=== Spatial Hash vs GridSpatialHash Benchmark ===\n");
    
    let width = 1000;
    let height = 1000;
    let cell_size = 16;
    let iterations = 5;
    
    println!("Grid: {}x{}, Cell size: {}px, Iterations: {}\n", width, height, cell_size, iterations);
    
    // Test with different particle counts
    let particle_counts = vec![1000, 5000, 10000, 50000, 100000];
    
    println!("{:<8} {:>12} {:>12} {:>12} {:>12} {:>10} {:>10}",
        "Particles", "Hash Build", "Grid Build", "Hash Query", "Grid Query", "Build SPx", "Query SPx");
    println!("{}", "-".repeat(90));
    
    let mut results = Vec::new();
    for count in &particle_counts {
        let result = run_comparison_benchmark(width, height, *count, iterations);
        println!("{:>8} | {:>10.3}ms | {:>10.3}ms | {:>10.3}ms | {:>10.3}ms | {:>8.2}x | {:>8.2}x",
            result.particles,
            result.hash_build_ms,
            result.grid_build_ms,
            result.hash_query_ms,
            result.grid_query_ms,
            result.speedup_build(),
            result.speedup_query()
        );
        results.push(result);
    }
    
    println!("{}\n", "-".repeat(90));
    
    // Summary
    println!("=== Summary ===\n");
    println!("GridSpatialHash provides better cache locality with pre-allocated arrays.");
    println!("Build speedup is most significant with larger particle counts.\n");
    
    let avg_build_speedup: f64 = results.iter().map(|r| r.speedup_build()).sum::<f64>() / results.len() as f64;
    let avg_query_speedup: f64 = results.iter().map(|r| r.speedup_query()).sum::<f64>() / results.len() as f64;
    
    println!("Average build speedup: {:.2}x", avg_build_speedup);
    println!("Average query speedup: {:.2}x", avg_query_speedup);
    
    // FPS estimation for GridSpatialHash (60fps budget = 16.67ms)
    println!("\n=== FPS Estimation (GridSpatialHash) ===\n");
    println!("{:<12} {:>12} {:>12} {:>12}",
        "Particles", "Build (ms)", "Query (ms)", "Est. FPS");
    println!("{}", "-".repeat(52));
    
    for result in &results {
        let total_ms = result.grid_build_ms + result.grid_query_ms;
        let fps = if total_ms > 0.0 { 1000.0 / total_ms } else { 999.0 };
        println!("{:>12} {:>12.3} {:>12.3} {:>12.1}",
            result.particles,
            result.grid_build_ms,
            result.grid_query_ms,
            fps
        );
    }
    
    println!("\nTarget: 100K @ 30fps (33.33ms budget), 500K @ 60fps (16.67ms budget)");
}