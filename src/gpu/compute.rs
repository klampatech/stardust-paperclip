//! GPU Compute Pipeline for Particle Simulation
//! 
//! Uses WebGPU compute shaders to parallelize particle physics calculations.
//! Each particle cell is processed independently across GPU threads.

use std::sync::Arc;

/// GPU compute pipeline status
#[derive(Debug, Clone)]
pub enum GpuStatus {
    /// GPU is initialized and ready
    Ready,
    /// GPU initialization in progress
    Initializing,
    /// GPU not available (no WebGPU support)
    Unavailable(String),
    /// GPU error occurred
    Error(String),
}

impl GpuStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, GpuStatus::Ready)
    }
}

/// GPU compute pipeline for particle simulation
#[cfg(feature = "gpu")]
pub struct GpuPipeline {
    /// WebGPU device (cloned from Arc)
    device: wgpu::Device,
    /// Compute pipeline for particle physics
    compute_pipeline: wgpu::ComputePipeline,
    /// Buffer for particle data
    particle_buffer: wgpu::Buffer,
    /// Uniform buffer for simulation parameters
    uniform_buffer: wgpu::Buffer,
    /// Grid dimensions
    grid_size: (u32, u32),
    /// Current status
    status: GpuStatus,
}

#[cfg(feature = "gpu")]
impl GpuPipeline {
    /// Create a new GPU pipeline
    pub async fn new(width: u32, height: u32) -> Result<Self, String> {
        // Request WebGPU adapter
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| "No WebGPU adapter available".to_string())?;

        // Request device
        let (device, _queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                    label: Some("falling_sand_gpu"),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to request device: {}", e))?;

        // Create shader module from embedded WGSL
        let shader_source = include_str!("shaders.wgsl");
        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("particle_compute"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create compute pipeline
        let compute_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("particle_pipeline"),
                layout: None,
                module: &shader_module,
                entry_point: "main",
            },
        );

        // Create particle buffer (u32 per cell for material + flags)
        let cell_count = (width * height) as usize;
        let particle_data_size = cell_count * std::mem::size_of::<u32>();
        
        let particle_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("particle_buffer"),
            size: particle_data_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create uniform buffer (grid size, tick count, etc.)
        let uniform_buffer_size = (std::mem::size_of::<SimulationUniforms>() as u64).max(4);
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size: uniform_buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            device,
            compute_pipeline,
            particle_buffer,
            uniform_buffer,
            grid_size: (width, height),
            status: GpuStatus::Ready,
        })
    }

    /// Update particle data from CPU grid
    #[allow(dead_code)]
    pub fn update_particles(&mut self, _data: &[u32]) {
        // Would use queue.write_buffer() to update particle_buffer
        // This is a placeholder for the actual implementation
    }

    /// Run one simulation tick on GPU
    #[allow(dead_code)]
    pub fn tick(&mut self) {
        // Submit compute dispatch to GPU
        // This is a placeholder - actual implementation would dispatch compute pass
    }

    /// Get current status
    pub fn status(&self) -> &GpuStatus {
        &self.status
    }

    /// Get grid dimensions
    pub fn grid_size(&self) -> (u32, u32) {
        self.grid_size
    }
}

/// Simulation uniforms passed to compute shader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct SimulationUniforms {
    /// Grid width
    width: u32,
    /// Grid height
    height: u32,
    /// Current tick count
    tick: u32,
    /// Padding for alignment
    _padding: u32,
}

/// Fallback CPU implementation when GPU not available
#[cfg(not(feature = "gpu"))]
pub struct GpuPipeline {
    status: GpuStatus,
    grid_size: (u32, u32),
}

#[cfg(not(feature = "gpu"))]
impl GpuPipeline {
    /// Create fallback pipeline (CPU-only)
    #[allow(dead_code)]
    pub fn new(_width: u32, _height: u32) -> Result<Self, String> {
        Ok(Self {
            status: GpuStatus::Unavailable("GPU feature not enabled".to_string()),
            grid_size: (0, 0),
        })
    }

    #[allow(dead_code)]
    pub fn status(&self) -> &GpuStatus {
        &self.status
    }

    #[allow(dead_code)]
    pub fn grid_size(&self) -> (u32, u32) {
        self.grid_size
    }
}