//! Falling Sand Simulation - Core Library
//! 
//! A high-performance particle simulation engine with spatial partitioning.
//! 
//! # Features
//! - Spatial partitioning with 64x64 chunks for 50,000+ particle support
//! - 13 material types with distinct physics behaviors
//! - Temperature system for heat simulation and phase changes
//! - Fire spread, lava heating, and material interactions
//! - Canvas2D rendering pipeline
//! - WebAssembly support via wasm-bindgen
//! 
//! # Materials (13)
//! - **Fluids**: Sand, Water, Oil, Ice, Ash (fall with gravity)
//! - **Risers**: Fire, Smoke, Steam (rise upward)
//! - **Solids**: Stone, Wood (static, immovable)
//! - **Special**: BlackHole (gravity well), Lava (hot, flows slowly)
//! 
//! # Temperature System
//! - Ice melts → Water when heated (T > 273K)
//! - Water boils → Steam when heated (T > 373K)
//! - Lava cools → Stone when not near heat sources
//! 
//! # Example
//! ```rust,ignore
//! use falling_sand::{Grid, GridSize, Material, Simulator};
//! 
//! let mut grid = Grid::new(GridSize::new(100, 100));
//! grid.spawn(50, 50, Material::Sand);
//! grid.spawn(50, 10, Material::Lava); // Heats nearby materials
//! 
//! let mut sim = Simulator::new();
//! sim.tick(&mut grid);
//! ```

// Re-export public API
pub use crate::grid::Grid;
pub use crate::grid::GridSize;
pub use crate::particle::{Material, Particle, ParticleFlags, BlackHoleProps};
pub use crate::chunk::{Chunk, ChunkedGrid, ChunkPos, CHUNK_SIZE};
pub use crate::simulation::Simulator;
pub use crate::renderer::{Renderer, TerminalRenderer, Color};
pub use crate::postprocessing::{PostProcessor, PostProcessingConfig, ColorGradingMode, ScreenShake};

// GPU compute pipeline (optional feature)
#[cfg(feature = "gpu")]
pub mod gpu;

// Internal modules
mod grid;
mod particle;
mod chunk;
mod simulation;
pub mod renderer;
pub mod postprocessing;

// WASM-specific exports
#[cfg(feature = "wasm")]
pub mod wasm_bindings;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(GridSize::new(10, 10));
        assert_eq!(grid.size().width, 10);
        assert_eq!(grid.size().height, 10);
    }
    
    #[test]
    fn test_particle_operations() {
        let mut grid = Grid::new(GridSize::new(5, 5));
        
        // Spawn a sand particle
        assert!(grid.spawn(2, 2, Material::Sand));
        
        // Check it's there
        let particle = grid.get(2, 2).unwrap();
        assert_eq!(particle.material, Material::Sand);
        
        // Try to spawn on occupied cell
        assert!(!grid.spawn(2, 2, Material::Water));
    }
    
    #[test]
    fn test_swap() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        
        grid.spawn(0, 0, Material::Sand);
        grid.spawn(1, 1, Material::Water);
        
        grid.swap(0, 0, 1, 1);
        
        assert_eq!(grid.get(0, 0).unwrap().material, Material::Water);
        assert_eq!(grid.get(1, 1).unwrap().material, Material::Sand);
    }
}
