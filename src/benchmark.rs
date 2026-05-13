//! Benchmark harness for spatial hashing performance
//! 
//! Tests spatial query performance for large particle counts.
//! Target: 100K particles @ 30fps, 500K particles @ 60fps

use crate::chunk::{SpatialHash, CHUNK_SIZE};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct SpatialBenchmarkConfig {
    /// Number of particles to test
    pub particle_counts: Vec<usize>,
    /// Number of iterations per test
    pub iterations: usize,
    /// Cell size for spatial hash
    pub cell_size: usize,
}

impl Default for SpatialBenchmarkConfig {
    fn default() -> Self {
        Self {
            particle_counts: vec![1000, 10000, 50000, 100000, 500000],
            iterations: 10,
            cell_size: 16, // Tuned for typical particle density
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Number of particles
    pub particle_count: usize,
    /// Time to build spatial hash (ms)
    pub build_time_ms: f64,
    /// Time for neighbor queries (ms)
    pub query_time_ms: f64,
    /// Average query time per neighbor lookup (ns)
    pub avg_query_ns: f64,
    /// Memory usage estimate (bytes per particle)
    pub memory_per_particle: f64,
}

impl BenchmarkResult {
    pub fn summary(&self) -> String {
        format!(
            "Particles: {:>8} | Build: {:>8.2}ms | Query: {:>8.2}ms | Avg: {:>6.0}ns/op | Memory: {:>5.0}B/particle",
            self.particle_count,
            self.build_time_ms,
            self.query_time_ms,
            self.avg_query_ns,
            self.memory_per_particle
        )
    }
}

pub struct SpatialBenchmark {
    config: SpatialBenchmarkConfig,
    results: Vec<BenchmarkResult>,
}

impl SpatialBenchmark {
    pub fn new(config: SpatialBenchmarkConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }
    
    /// Run all benchmarks
    pub fn run(&mut self, width: usize, height: usize) {
        println!("\n=== Spatial Hash Benchmark ===");
        println!("Grid: {}x{}, Cell size: {}", width, height, self.config.cell_size);
        println!("Iterations per test: {}\n", self.config.iterations);
        println!("{:<12} {:>10} {:>12} {:>12} {:>10} {:>12}",
            "Particles", "Build(ms)", "Query(ms)", "Avg(ns)", "Mem(B)", "FPS est.");
        println!("{}", "-".repeat(72));
        
        for &count in &self.config.particle_counts {
            let result = self.benchmark_single(width, height, count);
            println!("{}", result.summary());
            self.results.push(result);
        }
        
        println!("{}\n", "-".repeat(72));
    }
    
    /// Benchmark with a specific particle count
    fn benchmark_single(&self, width: usize, height: usize, count: usize) -> BenchmarkResult {
        let mut build_times = Vec::with_capacity(self.config.iterations);
        let mut query_times = Vec::with_capacity(self.config.iterations);
        
        for _ in 0..self.config.iterations {
            // Generate random positions
            let positions = generate_random_positions(count, width, height);
            
            // Benchmark build
            let start = Instant::now();
            let mut hash = SpatialHash::new(width, height, self.config.cell_size);
            for (x, y) in &positions {
                hash.insert(*x, *y);
            }
            let build_time = start.elapsed();
            build_times.push(build_time.as_secs_f64() * 1000.0);
            
            // Benchmark queries (sample queries)
            let start = Instant::now();
            let sample_size = (positions.len() as f32 * 0.1) as usize; // 10% sample
            let mut total_queries = 0;
            
            for (i, &(x, y)) in positions.iter().enumerate() {
                if i >= sample_size {
                    break;
                }
                let _ = hash.get_neighbors(x, y);
                total_queries += 1;
            }
            let query_time = start.elapsed();
            query_times.push(query_time.as_secs_f64() * 1000.0);
        }
        
        // Calculate averages
        let avg_build = build_times.iter().sum::<f64>() / build_times.len() as f64;
        let avg_query = query_times.iter().sum::<f64>() / query_times.len() as f64;
        let avg_queries = (count as f32 * 0.1) as f64;
        let avg_per_query = if avg_queries > 0.0 { 
            (avg_query * 1_000_000.0) / avg_queries 
        } else { 
            0.0 
        };
        
        // Estimate FPS (assuming 60fps budget of 16.67ms per frame)
        // FPS = 1 / (build_time + query_time * particles / sample_size / 1000)
        let frame_time_ms = avg_build + avg_query * (count as f64 / avg_queries);
        let fps = if frame_time_ms > 0.0 { 1000.0 / frame_time_ms } else { 999.0 };
        
        // Memory estimate (HashMap overhead + Vec storage)
        // Approx: 8 bytes per entry (key) + 16 bytes per position + HashMap overhead
        let memory_per_particle = 32.0; // Conservative estimate
        
        BenchmarkResult {
            particle_count: count,
            build_time_ms: avg_build,
            query_time_ms: avg_query,
            avg_query_ns: avg_per_query,
            memory_per_particle,
        }
    }
    
    /// Print summary table
    pub fn print_summary(&self) {
        println!("\n=== Summary ===");
        println!("Target: 100K @ 30fps minimum, 500K @ 60fps target\n");
        
        let mut meets_100k = false;
        let mut meets_500k = false;
        
        for result in &self.results {
            if result.particle_count >= 100000 {
                let frame_time = result.build_time_ms + result.query_time_ms;
                if frame_time <= 33.33 { // 30fps budget
                    meets_100k = true;
                }
            }
            if result.particle_count >= 500000 {
                let frame_time = result.build_time_ms + result.query_time_ms;
                if frame_time <= 16.67 { // 60fps budget
                    meets_500k = true;
                }
            }
        }
        
        println!("100K @ 30fps: {}", if meets_100k { "✓ PASS" } else { "○ OPTIMIZE" });
        println!("500K @ 60fps: {}", if meets_500k { "✓ PASS" } else { "○ OPTIMIZE" });
    }
    
    /// Get all results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

/// Generate random positions within grid bounds
fn generate_random_positions(count: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut positions = Vec::with_capacity(count);
    let mut hasher = DefaultHasher::new();
    
    // Simple pseudo-random based on position in sequence
    for i in 0..count {
        i.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Split hash into x and y coordinates
        let x = (hash % (width as u64)) as usize;
        let y = ((hash >> 16) % (height as u64)) as usize;
        
        positions.push((x, y));
        
        // Reset hasher for next iteration
        hasher = DefaultHasher::new();
    }
    
    positions
}

/// Run spatial query benchmark
pub fn run_spatial_benchmark(width: usize, height: usize) {
    let config = SpatialBenchmarkConfig::default();
    let mut benchmark = SpatialBenchmark::new(config);
    benchmark.run(width, height);
    benchmark.print_summary();
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    
    #[test]
    fn test_benchmark_small() {
        let config = SpatialBenchmarkConfig {
            particle_counts: vec![100, 1000],
            iterations: 3,
            cell_size: 16,
        };
        let mut benchmark = SpatialBenchmark::new(config);
        benchmark.run(100, 100);
        assert!(!benchmark.results().is_empty());
    }
    
    #[test]
    fn test_generate_positions() {
        let positions = generate_random_positions(100, 100, 100);
        assert_eq!(positions.len(), 100);
        
        // All positions should be within bounds
        for (x, y) in positions {
            assert!(x < 100);
            assert!(y < 100);
        }
    }
}