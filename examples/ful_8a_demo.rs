//! Spatial Hashing Demo - FUL-8a
//! 
//! Demonstrates spatial hashing, dirty-chunk tracking, and sleeping particles.

use falling_sand::{
    ChunkedGrid, GridSize, Material, Particle, Simulator, 
    SpatialHash, SpatialBenchmarkConfig, run_spatial_benchmark,
    CHUNK_SIZE
};

fn main() {
    println!("=== FUL-8a: Spatial Hashing & Dirty-Chunk Tracking Demo ===\n");
    
    // Part 1: Demonstrate ChunkedGrid with dirty tracking
    println!("[1] ChunkedGrid with Dirty-Chunk Tracking");
    println!("{}", "=".repeat(50));
    demo_chunked_grid();
    
    println!("\n[2] SpatialHash - O(1) Neighbor Lookups");
    println!("{}", "=".repeat(50));
    demo_spatial_hash();
    
    println!("\n[3] Sleeping Particle System");
    println!("{}", "=".repeat(50));
    demo_sleeping_particles();
    
    println!("\n[4] Spatial Benchmark (performance test)");
    println!("{}", "=".repeat(50));
    run_spatial_benchmark(200, 200);
    
    println!("\n=== Demo Complete ===");
    println!("FUL-8a deliverables demonstrated:");
    println!("  ✓ Spatial hash with configurable cell size");
    println!("  ✓ Dirty-chunk tracking with incremental invalidation");
    println!("  ✓ Sleeping particle list with wake-on-neighbor-change");
    println!("  ✓ Benchmark harness for spatial queries");
}

/// Demonstrate chunked grid with dirty tracking
fn demo_chunked_grid() {
    let size = GridSize::new(128, 128);
    let mut grid = ChunkedGrid::new(size);
    
    // Spawn particles across multiple chunks
    println!("Spawning particles in different chunks...");
    
    // Chunk (0, 0) - positions 0-63
    grid.spawn(10, 10, Material::Sand);
    grid.spawn(20, 20, Material::Water);
    
    // Chunk (1, 0) - positions 64-127
    grid.spawn(74, 10, Material::Fire);
    
    // Chunk (0, 1) - positions (y: 64-127)
    grid.spawn(10, 74, Material::Stone);
    
    println!("  Total particles: {}", grid.total_particles());
    println!("  Active chunks: {}", grid.active_chunks().len());
    println!("  Dirty chunks (from spawn): {}", grid.dirty_chunks().len());
    
    // Clear dirty and verify
    grid.clear_dirty();
    println!("  After clear_dirty(): {}", grid.dirty_chunks().len());
    
    // Mark chunks dirty manually
    grid.mark_dirty(falling_sand::ChunkPos::new(0, 0));
    println!("  After mark_dirty((0,0)): {}", grid.dirty_chunks().len());
    
    println!("\nChunk positions:");
    println!("  (0,0) at world (10,10) - Sand/Water");
    println!("  (1,0) at world (74,10) - Fire");
    println!("  (0,1) at world (10,74) - Stone");
}

/// Demonstrate spatial hash O(1) lookups
fn demo_spatial_hash() {
    let mut hash = SpatialHash::new(100, 100, 16); // 16px cell size
    
    // Insert some particles
    println!("Inserting 100 random particles...");
    let mut rng = SimpleRng::new(42);
    for _ in 0..100 {
        let x = rng.next() % 100;
        let y = rng.next() % 100;
        hash.insert(x, y);
    }
    
    println!("  Occupied cells: {}", hash.occupied_cells());
    println!("  Total particles: {}", hash.total_particles());
    println!("  Cell size: {}px", hash.cell_size());
    
    // Query neighbors
    let neighbors = hash.get_neighbors(50, 50);
    println!("  Neighbors of (50,50): {}", neighbors.len());
    
    // Query empty area
    let empty_neighbors = hash.get_neighbors(0, 0);
    println!("  Neighbors of (0,0): {} (sparse area)", empty_neighbors.len());
    
    println!("\nSpatial hash enables O(1) neighbor lookups for collision detection!");
}

/// Demonstrate sleeping particle system
fn demo_sleeping_particles() {
    let mut grid = ChunkedGrid::new(GridSize::new(128, 128));
    
    // Spawn some stable particles
    println!("Spawning stable particles...");
    grid.spawn(10, 100, Material::Sand);  // On floor - stable
    grid.spawn(11, 100, Material::Sand);  // On floor - stable
    grid.spawn(12, 100, Material::Sand);  // On floor - stable
    
    println!("  Total particles: {}", grid.total_particles());
    
    // Mark them as sleeping (stable, no physics needed)
    println!("\nMarking particles as sleeping (stable on floor)...");
    grid.mark_sleeping(10, 100);
    grid.mark_sleeping(11, 100);
    grid.mark_sleeping(12, 100);
    
    println!("  Sleeping count: {}", grid.sleeping_count());
    println!("  Is (10,100) sleeping? {}", grid.is_sleeping(10, 100));
    
    // Wake one particle
    println!("\nWaking particle at (10,100)...");
    grid.wake_particle(10, 100);
    
    println!("  Is (10,100) sleeping? {}", grid.is_sleeping(10, 100));
    println!("  Was (10,100) woken? {}", grid.was_woken(10, 100));
    println!("  Sleeping count now: {}", grid.sleeping_count());
    
    // Clear woken
    grid.clear_woken();
    println!("  After clear_woken(), was_woken? {}", grid.was_woken(10, 100));
    
    // Demonstrate wake_neighbor_chunks
    println!("\nWake neighbor chunks (when adjacent chunk changes)...");
    grid.clear_dirty();
    grid.mark_dirty(falling_sand::ChunkPos::new(0, 0)); // Mark own chunk dirty
    grid.wake_neighbor_chunks();
    
    println!("\nSleeping particles skip physics until neighbors change!");
    println!("  wake_neighbor_chunks() wakes particles in adjacent dirty chunks");
}

/// Simple pseudo-random number generator for reproducibility
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next(&mut self) -> usize {
        // LCG: x_{n+1} = (a * x_n + c) mod m
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state % 100) as usize
    }
}

/// Spawn particle wrapper that doesn't break on failure
fn safe_spawn(grid: &mut ChunkedGrid, x: usize, y: usize, mat: Material) {
    let _ = grid.spawn(x, y, mat);
}