//! GPU Compute Shader Pipeline
//! 
//! Provides GPU-accelerated particle simulation using WebGPU compute shaders.
//! Falls back to CPU simulation when GPU is unavailable.
//! 
//! Features:
//! - Compute shaders for particle physics
//! - Instanced rendering with culling
//! - LOD system for distant particles
//! - Frame timing and profiling

#[cfg(feature = "gpu")]
mod compute;

#[cfg(feature = "gpu")]
pub use compute::*;

// Render pipeline
pub mod render;

/// Encode a Material to GPU u32 format (5 bits for material ID)
pub fn encode_material(material: crate::particle::Material) -> u32 {
    material as u32
}

/// Decode GPU u32 format to Material
pub fn decode_material(encoded: u32) -> crate::particle::Material {
    let id = encoded & 0x1F;
    match id {
        0 => crate::particle::Material::Air,
        1 => crate::particle::Material::Sand,
        2 => crate::particle::Material::Water,
        3 => crate::particle::Material::Stone,
        4 => crate::particle::Material::Fire,
        5 => crate::particle::Material::Smoke,
        6 => crate::particle::Material::BlackHole,
        7 => crate::particle::Material::Steam,
        8 => crate::particle::Material::Ice,
        9 => crate::particle::Material::Oil,
        10 => crate::particle::Material::Wood,
        11 => crate::particle::Material::Lava,
        12 => crate::particle::Material::Ash,
        _ => crate::particle::Material::Air,
    }
}

/// Encode grid to GPU buffer format
pub fn encode_grid(grid: &crate::Grid) -> Vec<u32> {
    let size = grid.size();
    let mut data = Vec::with_capacity(size.width * size.height);
    
    for y in 0..size.height {
        for x in 0..size.width {
            let encoded = grid.get(x, y)
                .map(|p| encode_material(p.material))
                .unwrap_or(0);
            data.push(encoded);
        }
    }
    
    data
}

/// GPU status enum
#[derive(Debug, Clone)]
pub enum GpuStatus {
    Ready,
    Initializing,
    Unavailable(String),
    Error(String),
}

impl GpuStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, GpuStatus::Ready)
    }
    
    pub fn message(&self) -> String {
        match self {
            GpuStatus::Ready => "GPU ready".to_string(),
            GpuStatus::Initializing => "Initializing...".to_string(),
            GpuStatus::Unavailable(msg) => msg.clone(),
            GpuStatus::Error(msg) => format!("GPU error: {}", msg),
        }
    }
}

/// GPU pipeline (stub when GPU not enabled)
pub struct GpuPipeline {
    status: GpuStatus,
    grid_size: (u32, u32),
    tick: u32,
}

impl GpuPipeline {
    pub fn new(_width: u32, _height: u32) -> Result<Self, String> {
        Ok(Self {
            status: GpuStatus::Unavailable("GPU feature not enabled".to_string()),
            grid_size: (0, 0),
            tick: 0,
        })
    }

    pub fn encode_grid(&mut self, _grid: &[u32]) {}
    pub fn tick(&mut self) { self.tick += 1; }
    pub fn submit(&mut self, _commands: ()) {}
    pub fn status(&self) -> &GpuStatus { &self.status }
    pub fn grid_size(&self) -> (u32, u32) { self.grid_size }
    pub fn tick_count(&self) -> u32 { self.tick }
    pub fn cell_count(&self) -> usize { (self.grid_size.0 * self.grid_size.1) as usize }
}

/// CPU-only GPU simulator
pub struct GpuSimulator {
    cpu_sim: crate::Simulator,
    grid_size: (u32, u32),
    gpu_enabled: bool,
}

impl GpuSimulator {
    pub fn new_cpu(width: u32, height: u32) -> Self {
        Self {
            cpu_sim: crate::Simulator::new(),
            grid_size: (width, height),
            gpu_enabled: false,
        }
    }

    pub fn tick(&mut self, grid: &mut crate::Grid) {
        self.cpu_sim.tick(grid);
    }

    pub fn uses_gpu(&self) -> bool {
        self.gpu_enabled
    }
}