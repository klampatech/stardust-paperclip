//! GPU Compute Shader Benchmark
//! 
//! Async benchmark for GPU compute pipeline performance.
//! Requires tokio runtime for WebGPU initialization.

#[cfg(feature = "gpu")]
use falling_sand::gpu::{GpuPipeline, GpuStatus};
use std::time::Instant;

#[cfg(feature = "gpu")]
#[tokio::main]
async fn main() {
    println!("GPU Compute Shader Benchmark");
    println!("============================");
    println!();
    
    // Initialize GPU pipeline
    let grid_width = 256u32;
    let grid_height = 256u32;
    
    match GpuPipeline::new(grid_width, grid_height).await {
        Ok(mut pipeline) => {
            println!("✓ GPU Pipeline initialized");
            println!("  Grid: {}x{} ({} cells)", grid_width, grid_height, pipeline.cell_count());
            println!("  Status: {}", pipeline.status().message());
            println!();
            
            // Run benchmark
            run_benchmark(&mut pipeline, grid_width, grid_height);
        }
        Err(e) => {
            println!("⚠ GPU initialization failed: {}", e);
            println!("  Falling back to CPU benchmark...");
            println!();
            run_cpu_fallback();
        }
    }
}

#[cfg(feature = "gpu")]
fn run_benchmark(pipeline: &mut GpuPipeline, width: u32, height: u32) {
    use falling_sand::{Grid, GridSize, Material, Simulator};
    
    println!("=== GPU Benchmark ===");
    println!();
    
    // Create sample data
    let cell_count = (width * height) as usize;
    let mut grid_data = vec![0u32; cell_count];
    
    // Encode particles (10% density)
    let particle_count = cell_count / 10;
    for i in 0..particle_count {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        let idx = (y * width + x) as usize;
        // Encode material (5 bits) + flags (bits 5-31)
        grid_data[idx] = (i % 5) as u32 + 1; // Materials 1-5
    }
    
    println!("Encoded {} particles", particle_count);
    
    // Encode to GPU
    pipeline.encode_grid(&grid_data);
    
    // Run 1000 GPU ticks
    let ticks = 1000u32;
    println!("Running {} GPU ticks...", ticks);
    
    let start = Instant::now();
    
    for _ in 0..ticks {
        let cmd = pipeline.tick();
        pipeline.submit(cmd);
    }
    
    let elapsed = start.elapsed();
    let ms_per_tick = elapsed.as_secs_f64() / ticks as f64 * 1000.0;
    
    println!();
    println!("Results:");
    println!("  Total time: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
    println!("  Per tick: {:.3}ms", ms_per_tick);
    println!("  Throughput: {:.0} ticks/sec", 1000.0 / ms_per_tick);
    println!();
    
    // Compare with CPU
    println!("=== CPU Comparison ===");
    run_cpu_benchmark(GridSize::new(width as usize, height as usize), 100, "CPU");
}

#[cfg(feature = "gpu")]
fn run_cpu_benchmark(size: falling_sand::GridSize, ticks: u32, name: &str) {
    use falling_sand::{Grid, Material, Simulator};
    
    println!("{} Benchmark: {}x{}", name, size.width, size.height);
    println!("----------------------------------------");
    
    let mut grid = Grid::new(size);
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
    
    println!("Particles: {}", grid.particle_count());
    
    let mut sim = Simulator::new();
    let start = Instant::now();
    
    for _ in 0..ticks {
        sim.tick(&mut grid);
    }
    
    let elapsed = start.elapsed();
    let ms_per_tick = elapsed.as_secs_f64() / ticks as f64 * 1000.0;
    
    println!("Time: {:.2}ms total", elapsed.as_secs_f64() * 1000.0);
    println!("Per tick: {:.3}ms", ms_per_tick);
    println!("Throughput: {:.0} ticks/sec", 1000.0 / ms_per_tick);
    println!();
}

#[cfg(feature = "gpu")]
fn run_cpu_fallback() {
    run_cpu_benchmark(falling_sand::GridSize::new(256, 256), 500, "CPU");
}

#[cfg(not(feature = "gpu"))]
fn main() {
    println!("GPU feature not enabled");
    println!("Run with: cargo run --example gpu_bench --features gpu");
}