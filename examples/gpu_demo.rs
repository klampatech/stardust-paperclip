//! GPU Compute Shader Demo
//! 
//! Demonstrates the GPU compute pipeline for particle simulation.
//! Falls back to CPU simulation if GPU not available.

use falling_sand::{Grid, GridSize, Material, Particle, Simulator};
use std::time::Instant;

fn main() {
    println!("GPU Compute Shader Pipeline Demo");
    println!("==================================");
    println!();
    
    // Check GPU availability
    #[cfg(feature = "gpu")]
    {
        println!("GPU feature enabled - checking WebGPU availability...");
        
        // Try to initialize GPU pipeline
        let grid_width = 128u32;
        let grid_height = 128u32;
        
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        let pipeline_result = rt.block_on(async {
            falling_sand::gpu::GpuPipeline::new(grid_width, grid_height).await
        });
        
        match pipeline_result {
            Ok(pipeline) => {
                println!("✓ GPU Pipeline initialized successfully");
                println!("  Grid size: {}x{}", grid_width, grid_height);
                println!("  Status: {:?}", pipeline.status());
            }
            Err(e) => {
                println!("⚠ GPU Pipeline initialization failed: {}", e);
                println!("  Falling back to CPU simulation");
            }
        }
    }
    
    #[cfg(not(feature = "gpu"))]
    {
        println!("GPU feature not enabled");
        println!("Run with: cargo run --example gpu_demo --features gpu");
        println!();
        println!("Falling back to CPU simulation for demonstration...");
    }
    
    println!();
    println!("=== CPU Simulation Performance Test ===");
    
    // Run performance comparison
    run_cpu_benchmark(GridSize::new(256, 256), 1000);
    run_cpu_benchmark(GridSize::new(512, 512), 500);
    
    println!();
    println!("=== Particle Interaction Demo ===");
    run_demo_simulation();
}

fn run_cpu_benchmark(size: GridSize, ticks: u32) {
    println!("\nBenchmark: {}x{} grid, {} ticks", size.width, size.height, ticks);
    println!("----------------------------------------");
    
    // Create grid with random particles
    let mut grid = Grid::new(size);
    
    // Fill with various particles
    let particle_count = (size.width * size.height) / 10; // 10% density
    println!("Spawning {} particles...", particle_count);
    
    for i in 0..particle_count {
        let x = (i as usize) % size.width;
        let y = (i as usize) / size.width;
        let material = match (i % 5) {
            0 => Material::Sand,
            1 => Material::Water,
            2 => Material::Stone,
            3 => Material::Fire,
            _ => Material::Oil,
        };
        let _ = grid.spawn(x, y, material);
    }
    
    println!("Particles: {}", grid.particle_count());
    
    // Run simulation and time it
    let mut sim = Simulator::new();
    let start = Instant::now();
    
    for _ in 0..ticks {
        sim.tick(&mut grid);
    }
    
    let elapsed = start.elapsed();
    let ms_per_tick = elapsed.as_secs_f64() / ticks as f64 * 1000.0;
    
    println!("Time: {:.2}ms total", elapsed.as_secs_f64() * 1000.0);
    println!("Average: {:.3}ms per tick", ms_per_tick);
    println!("Throughput: {:.0} ticks/second", 1000.0 / ms_per_tick);
}

fn run_demo_simulation() {
    let size = GridSize::new(40, 30);
    let mut grid = Grid::new(size);
    
    // Create container
    for x in 5..=35 {
        grid.set(x, 26, Particle::new(Material::Stone));
    }
    for y in 5..=26 {
        grid.set(5, y, Particle::new(Material::Stone));
        grid.set(35, y, Particle::new(Material::Stone));
    }
    
    // Spawn sand pile
    for x in 15..=25 {
        for y in 2..=5 {
            if (x - 20).abs() + (y - 3) < 4 {
                let _ = grid.spawn(x, y, Material::Sand);
            }
        }
    }
    
    // Spawn water
    for x in 10..=14 {
        let _ = grid.spawn(x, 3, Material::Water);
    }
    
    // Spawn fire
    let _ = grid.spawn(20, 10, Material::Fire);
    let _ = grid.spawn(22, 10, Material::Fire);
    
    // Place some oil
    for x in 28..=32 {
        let _ = grid.spawn(x, 4, Material::Oil);
    }
    
    println!("Initial state:");
    print_grid(&grid);
    
    let mut sim = Simulator::new();
    
    for tick in 1..=6 {
        sim.tick(&mut grid);
        println!("\nTick {}:", tick);
        print_grid(&grid);
    }
    
    println!("\n✓ Simulation complete");
    println!("  Fire rises and can ignite oil");
    println!("  Water falls and flows");
    println!("  Sand piles on surfaces");
    println!("  GPU pipeline can accelerate this for larger grids");
}

fn print_grid(grid: &Grid) {
    let (width, height) = (grid.size().width, grid.size().height);
    
    println!("+{}+", "-".repeat(width));
    
    for y in 0..height {
        let mut row = String::from("|");
        for x in 0..width {
            let ch = grid.get(x, y)
                .map(|p| match p.material {
                    Material::Air => ' ',
                    Material::Sand => '°',
                    Material::Water => '~',
                    Material::Stone => '#',
                    Material::Fire => '*',
                    Material::Smoke => '@',
                    Material::BlackHole => '●',
                    Material::Steam => '≈',
                    Material::Ice => '▒',
                    Material::Oil => '█',
                    Material::Wood => '▓',
                    Material::Lava => '†',
                    Material::Ash => '·',
                })
                .unwrap_or('?');
            row.push(ch);
        }
        row.push('|');
        println!("{}", row);
    }
    
    println!("+{}+", "-".repeat(width));
}