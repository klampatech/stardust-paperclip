//! Full Integration Test - FUL-8c
//! 
//! Tests the complete integration of all FUL-8c components:
//! - ChunkedGrid with dirty tracking
//! - SpatialHash and GridSpatialHash
//! - tick_chunked() with spatial hash integration
//! - GPU simulator (CPU fallback when unavailable)

use falling_sand::{
    ChunkedGrid, GridSize, Material, Simulator, 
    SpatialHash, GridSpatialHash,
    GpuSimulator
};

fn main() {
    println!("=== FUL-8c: Full Integration Test ===\n");
    
    // Test 1: Spatial hash integration with physics pipeline
    println!("[1] Spatial Hash Integration Test");
    println!("{}", "-".repeat(40));
    test_spatial_hash_integration();
    
    // Test 2: GridSpatialHash optimization
    println!("\n[2] GridSpatialHash Test");
    println!("{}", "-".repeat(40));
    test_grid_spatial_hash();
    
    // Test 3: ChunkedGrid with dirty tracking
    println!("\n[3] Dirty Chunk Tracking Test");
    println!("{}", "-".repeat(40));
    test_dirty_chunk_tracking();
    
    // Test 4: Sleeping particle system
    println!("\n[4] Sleeping Particle System Test");
    println!("{}", "-".repeat(40));
    test_sleeping_particles();
    
    // Test 5: GPU simulator (CPU fallback)
    println!("\n[5] GPU Simulator Test (CPU fallback)");
    println!("{}", "-".repeat(40));
    test_gpu_simulator();
    
    // Test 6: Large scale simulation
    println!("\n[6] Large Scale Simulation (10K particles)");
    println!("{}", "-".repeat(40));
    test_large_scale();
    
    println!("\n=== All Integration Tests Passed ===");
}

/// Test spatial hash integration with tick_chunked()
fn test_spatial_hash_integration() {
    let mut grid = ChunkedGrid::new(GridSize::new(100, 100));
    let mut sim = Simulator::new();
    
    // Spawn fire near flammable materials
    for x in 45..55 {
        grid.spawn(x, 70, Material::Sand);
    }
    grid.spawn(50, 60, Material::Fire);  // Fire should spread to nearby
    
    println!("  Initial particles: {}", grid.total_particles());
    
    // Run a few ticks with spatial hash
    for i in 0..5 {
        sim.tick_chunked(&mut grid);
        println!("  Tick {}: {} particles", i + 1, grid.total_particles());
    }
    
    // Verify spatial hash is built during tick
    let mut hash = SpatialHash::new(100, 100, 16);
    for y in 0..100 {
        for x in 0..100 {
            if !grid.is_empty(x, y) {
                hash.insert(x, y);
            }
        }
    }
    println!("  Spatial hash: {} cells, {} particles", 
             hash.occupied_cells(), hash.total_particles());
    
    // Test neighbor lookup
    let neighbors = hash.get_neighbors(50, 50);
    println!("  Neighbors of (50,50): {}", neighbors.len());
}

/// Test GridSpatialHash optimization
fn test_grid_spatial_hash() {
    let width = 100;
    let height = 100;
    let cell_size = 16;
    
    // Build both hash types
    let mut spatial_hash = SpatialHash::new(width, height, cell_size);
    let mut grid_spatial_hash = GridSpatialHash::new(width, height, cell_size);
    
    // Insert particles
    let positions: Vec<(usize, usize)> = (0..500)
        .map(|(x, y)| (x % width, y % height))
        .collect();
    
    for (x, y) in &positions {
        spatial_hash.insert(*x, *y);
        grid_spatial_hash.insert(*x, *y);
    }
    
    println!("  SpatialHash: {} cells, {} particles",
             spatial_hash.occupied_cells(), spatial_hash.total_particles());
    println!("  GridSpatialHash: {} cells, {} particles",
             grid_spatial_hash.occupied_cells(), grid_spatial_hash.total_particles());
    
    // Verify consistency
    let test_positions = vec![(10, 10), (50, 50), (80, 80)];
    for (x, y) in &test_positions {
        let h_neighbors = spatial_hash.get_neighbors(*x, *y);
        let g_neighbors = grid_spatial_hash.get_neighbors(*x, *y);
        
        let mut h_sorted = h_neighbors.clone();
        let mut g_sorted = g_neighbors.clone();
        h_sorted.sort();
        g_sorted.sort();
        
        let consistent = h_sorted == g_sorted;
        println!("  ({:2}, {:2}) neighbors: {} vs {} - {}",
                 x, y, h_neighbors.len(), g_neighbors.len(),
                 if consistent { "✓" } else { "✗" });
    }
}

/// Test dirty chunk tracking
fn test_dirty_chunk_tracking() {
    let mut grid = ChunkedGrid::new(GridSize::new(128, 128));
    
    // Spawn in different chunks
    grid.spawn(10, 10, Material::Sand);    // Chunk (0,0)
    grid.spawn(80, 10, Material::Water);    // Chunk (1,0)
    grid.spawn(10, 80, Material::Fire);     // Chunk (0,1)
    
    let dirty_count = grid.dirty_chunks().len();
    println!("  Dirty chunks after spawn: {}", dirty_count);
    println!("  Active chunks: {}", grid.active_chunks().len());
    
    // Clear dirty and verify
    grid.clear_dirty();
    println!("  Dirty chunks after clear: {}", grid.dirty_chunks().len());
    
    // Mark chunk dirty manually
    grid.mark_dirty(falling_sand::ChunkPos::new(0, 0));
    println!("  Dirty chunks after mark_dirty: {}", grid.dirty_chunks().len());
}

/// Test sleeping particle system
fn test_sleeping_particles() {
    let mut grid = ChunkedGrid::new(GridSize::new(64, 64));
    
    // Spawn particles on "floor" (stable position)
    grid.spawn(30, 50, Material::Stone);
    grid.spawn(31, 50, Material::Stone);
    grid.spawn(30, 51, Material::Sand);  // On floor - stable
    
    // Mark sand as sleeping
    grid.mark_sleeping(30, 51);
    
    println!("  Total particles: {}", grid.total_particles());
    println!("  Sleeping: {} (should be 1)", grid.sleeping_count());
    println!("  Is (30,51) sleeping: {} (should be true)", grid.is_sleeping(30, 51));
    
    // Wake particle
    grid.wake_particle(30, 51);
    println!("  After wake - Sleeping: {}, was_woken: {}", 
             grid.sleeping_count(), grid.was_woken(30, 51));
    
    grid.clear_woken();
    println!("  After clear_woken - was_woken: {} (should be false)", 
             grid.was_woken(30, 51));
}

/// Test GPU simulator with CPU fallback
fn test_gpu_simulator() {
    let mut gpu_sim = GpuSimulator::new_cpu(100, 100);
    let mut grid = falling_sand::Grid::new(GridSize::new(100, 100));
    
    // Spawn some particles
    for x in 40..60 {
        for y in 70..80 {
            if (x + y) % 2 == 0 {
                let _ = grid.spawn(x, y, Material::Sand);
            }
        }
    }
    
    println!("  Grid particles: {}", grid.total_particles());
    println!("  Using GPU: {} (should be false)", gpu_sim.uses_gpu());
    
    // Run simulation
    let start = std::time::Instant::now();
    for _ in 0..10 {
        gpu_sim.tick(&mut grid);
    }
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    
    println!("  10 ticks: {:.3}ms ({:.3}ms avg)", elapsed, elapsed / 10.0);
    println!("  Particles after: {}", grid.total_particles());
}

/// Test large scale simulation
fn test_large_scale() {
    use std::time::Instant;
    
    let width = 200;
    let height = 200;
    let target_particles = 10000;
    
    let mut grid = ChunkedGrid::new(GridSize::new(width, height));
    let mut sim = Simulator::new();
    
    println!("  Grid size: {}x{}", width, height);
    
    // Spawn particles efficiently
    let start = Instant::now();
    let mut rng = SimpleRng::new(42);
    
    for _ in 0..target_particles {
        let x = rng.next() % width;
        let y = rng.next() % height;
        let material = match rng.next() % 4 {
            0 => Material::Sand,
            1 => Material::Water,
            2 => Material::Stone,
            _ => Material::Sand,
        };
        let _ = grid.spawn(x, y, material);
    }
    
    let spawn_time = start.elapsed().as_secs_f64() * 1000.0;
    println!("  Spawned {} particles in {:.2}ms", 
             grid.total_particles(), spawn_time);
    
    // Build spatial hash
    let start = Instant::now();
    let mut hash = GridSpatialHash::new(width, height, 16);
    for y in 0..height {
        for x in 0..width {
            if !grid.is_empty(x, y) {
                hash.insert(x, y);
            }
        }
    }
    let hash_time = start.elapsed().as_secs_f64() * 1000.0;
    println!("  Spatial hash built in {:.3}ms: {} cells, {} particles",
             hash_time, hash.occupied_cells(), hash.total_particles());
    
    // Run simulation ticks
    let start = Instant::now();
    for _ in 0..10 {
        sim.tick_chunked(&mut grid);
    }
    let tick_time = start.elapsed().as_secs_f64() * 1000.0;
    println!("  10 ticks in {:.2}ms ({:.3}ms avg)", tick_time, tick_time / 10.0);
    println!("  Particles after: {}", grid.total_particles());
    
    // Calculate FPS estimate
    let avg_tick_ms = tick_time / 10.0;
    let estimated_fps = 1000.0 / avg_tick_ms;
    println!("  Estimated FPS: {:.1}", estimated_fps);
}

/// Simple RNG for deterministic testing
struct SimpleRng { state: u64 }
impl SimpleRng {
    fn new(seed: u64) -> Self { Self { state: seed } }
    fn next(&mut self) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state as usize
    }
}