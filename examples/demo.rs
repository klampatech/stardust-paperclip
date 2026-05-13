//! Falling Sand Simulation - Demo
//! 
//! Demonstrates the basic physics simulation with ASCII output.

use falling_sand::{Grid, GridSize, Material, Particle, Simulator};

fn main() {
    println!("Falling Sand Simulation - Phase 1");
    println!("=================================");
    println!("Legend: °=Sand, ~=Water, #=Stone, *=Fire, @=Smoke");
    println!();
    
    // Create a small test grid
    let size = GridSize::new(24, 18);
    let mut grid = Grid::new(size);
    
    // Create a container floor and walls
    for x in 4..=20 {
        grid.set(x, 14, Particle::new(Material::Stone));
    }
    for y in 5..=14 {
        grid.set(4, y, Particle::new(Material::Stone));
        grid.set(20, y, Particle::new(Material::Stone));
    }
    
    // Add some sand
    for _ in 0..15 {
        let x = 8 + (rand_usize() % 9);
        let _ = grid.spawn(x, 2, Material::Sand);
    }
    
    // Add some water
    for x in 16..=18 {
        let _ = grid.spawn(x, 3, Material::Water);
    }
    
    // Add some fire
    for _ in 0..3 {
        let x = 12 + (rand_usize() % 3);
        let _ = grid.spawn(x, 4, Material::Fire);
    }
    
    println!("Initial state:");
    print_grid(&grid);
    
    // Run simulation
    let mut sim = Simulator::new();
    for tick in 1..=8 {
        sim.tick(&mut grid);
        println!("\nTick {}:", tick);
        print_grid(&grid);
    }
    
    println!("\n=== Simulation Complete ===");
    println!("Fire rises, spreads to flammable materials.");
    println!("Water extinguishes fire.");
    println!("Smoke rises and dissipates.");
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
                })
                .unwrap_or('?');
            row.push(ch);
        }
        row.push('|');
        println!("{}", row);
    }
    
    println!("+{}+", "-".repeat(width));
}

fn rand_usize() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize)
        .unwrap_or(0)
}
