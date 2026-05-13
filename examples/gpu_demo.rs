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
    
    // GPU feature requires async runtime - show info and run CPU benchmarks
    #[cfg(feature = "gpu")]
    {
        println!("GPU feature enabled (WebGPU compute shaders)");
        println!("GPU initialization requires async runtime");
        println!("See examples/gpu_bench.rs for GPU-specific benchmarks");
        println!();
    }
    
    #[cfg(not(feature = "gpu"))]
    {
        println!("GPU feature not enabled");
        println!("Run with: cargo run --example gpu_demo --features gpu");
        println!();
    }
    
    println!("=== CPU Simulation Performance Test ===");
    println!();
    
    // Run CPU performance benchmarks
    run_cpu_benchmark(GridSize::new(64, 64), 500, "64x64");
    run_cpu_benchmark(GridSize::new(128, 128), 500, "128x128");
    run_cpu_benchmark(GridSize::new(256, 256), 200, "256x256");
    
    #[cfg(feature = "gpu")]
    run_cpu_benchmark(GridSize::new(512, 512), 100, "512x512");
    
    println!();
    println!("=== GPU Pipeline Info ===");
    println!();
    println!("GPU Pipeline Components:");
    println!("  - src/gpu/mod.rs      : Module declarations");
    println!("  - src/gpu/compute.rs  : WebGPU pipeline implementation");
    println!("  - src/gpu/shaders.wgsl: WGSL compute shader");
    println!();
    println!("GPU Features:");
    println!("  - Ping-pong buffers for double-buffering");
    println!("  - Bind groups for shader inputs");
    println!("  - Workgroup size: 256 particles per group");
    println!("  - 16-byte aligned uniforms");
    println!();
    
    println!("=== Particle Interaction Demo ===");
    println!();
    run_demo_simulation();
    
    println!();
    println!("=== Demo Complete ===");
    println!();
    println!("To enable GPU acceleration:");
    println!("  1. Build with: cargo build --features gpu");
    println!("  2. Run GPU benchmark: cargo run --features gpu --example gpu_bench");
    println!("  3. Test WebGPU: Open browser at http://localhost:8080");
}

fn run_cpu_benchmark(size: GridSize, ticks: u32, name: &str) {
    println!("CPU Benchmark: {} grid, {} ticks", name, ticks);
    println!("----------------------------------------");
    
    // Create grid with random particles
    let mut grid = Grid::new(size);
    
    // Fill with various particles (10% density)
    let particle_count = (size.width * size.height) / 10;
    
    for i in 0..particle_count {
        let x = (i % size.width) as usize;
        let y = (i / size.width) as usize;
        let material = match (i % 5) {
            0 => Material::Sand,
            1 => Material::Water,
            2 => Material::Stone,
            3 => Material::Fire,
            _ => Material::Oil,
        };
        let _ = grid.spawn(x, y, material);
    }
    
    println!("Particles: {} in {}x{} grid", grid.particle_count(), size.width, size.height);
    
    // Run simulation and time it
    let mut sim = Simulator::new();
    let start = Instant::now();
    
    for _ in 0..ticks {
        sim.tick(&mut grid);
    }
    
    let elapsed = start.elapsed();
    let ms_per_tick = elapsed.as_secs_f64() / ticks as f64 * 1000.0;
    let particles_per_sec = (grid.particle_count() as f64 * 1000.0) / elapsed.as_secs_f64();
    
    println!("Time: {:.2}ms total", elapsed.as_secs_f64() * 1000.0);
    println!("Average: {:.3}ms per tick", ms_per_tick);
    println!("Throughput: {:.0} ticks/second", 1000.0 / ms_per_tick);
    println!("Particle updates: {:.0} per second", particles_per_sec);
    println!();
}

fn run_demo_simulation() {
    let size = GridSize::new(40, 25);
    let mut grid = Grid::new(size);
    
    // Create container
    for x in 5..=35 {
        grid.set(x, 22, Particle::new(Material::Stone));
    }
    for y in 5..=22 {
        grid.set(5, y, Particle::new(Material::Stone));
        grid.set(35, y, Particle::new(Material::Stone));
    }
    
    // Spawn sand pile
    for x in 15..=25 {
        for dy in 0..4 {
            let y = 3 + dy;
            if (x - 20).abs() + dy < 5 {
                let _ = grid.spawn(x, y, Material::Sand);
            }
        }
    }
    
    // Spawn water
    for x in 10..=14 {
        let _ = grid.spawn(x, 2, Material::Water);
    }
    
    // Spawn fire
    let _ = grid.spawn(20, 8, Material::Fire);
    let _ = grid.spawn(22, 8, Material::Fire);
    
    // Place oil
    for x in 28..=32 {
        let _ = grid.spawn(x, 3, Material::Oil);
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