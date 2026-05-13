//! GPU Compute Pipeline for Particle Simulation
//! 
//! Uses WebGPU compute shaders to parallelize particle physics calculations.
//! Each particle cell is processed independently across GPU threads.
//! 
//! Integration with CPU simulation via encode/decode.

use crate::particle::Material;

#[cfg(feature = "gpu")]
use wgpu::{Buffer, CommandBuffer, Device, Queue};

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
    
    pub fn message(&self) -> String {
        match self {
            GpuStatus::Ready => "GPU ready".to_string(),
            GpuStatus::Initializing => "Initializing...".to_string(),
            GpuStatus::Unavailable(msg) => msg.clone(),
            GpuStatus::Error(msg) => format!("GPU error: {}", msg),
        }
    }
}

/// GPU compute pipeline for particle simulation
#[cfg(feature = "gpu")]
pub struct GpuPipeline {
    /// WebGPU device
    device: Device,
    /// Command queue for GPU operations
    queue: Queue,
    /// Compute pipeline for particle physics
    compute_pipeline: wgpu::ComputePipeline,
    /// Input particle buffer (current state)
    particle_buffer_in: Buffer,
    /// Output particle buffer (new state)
    particle_buffer_out: Buffer,
    /// Uniform buffer for simulation parameters
    uniform_buffer: Buffer,
    /// Bind group for compute shader (ping)
    bind_group: wgpu::BindGroup,
    /// Alternate bind group for ping-pong (pong)
    bind_group_alt: wgpu::BindGroup,
    /// Grid dimensions
    grid_size: (u32, u32),
    /// Current tick count
    tick: u32,
    /// Current ping-pong index (0 or 1)
    ping_pong: usize,
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

        // Request device and queue
        let (device, queue) = adapter
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

        // Create compute pipeline with explicit layout
        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("compute_layout"),
                bind_group_layouts: &[
                    &Self::create_bind_group_layout(&device),
                ],
                push_constant_ranges: &[],
            },
        );
        
        let compute_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: Some("particle_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            },
        );

        // Calculate buffer sizes
        let cell_count = (width * height) as usize;
        let particle_data_size = cell_count * std::mem::size_of::<u32>();
        
        // Create input particle buffer (read-only in shader)
        let particle_buffer_in = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("particle_buffer_in"),
            size: particle_data_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create output particle buffer (read-write in shader)
        let particle_buffer_out = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("particle_buffer_out"),
            size: particle_data_size as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create uniform buffer (16-byte aligned)
        let uniform_buffer_size = (std::mem::size_of::<SimulationUniforms>() as u64).max(16);
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size: uniform_buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind groups for ping-pong
        let bind_group = Self::create_bind_group(
            &device,
            &compute_pipeline,
            &uniform_buffer,
            &particle_buffer_in,
            &particle_buffer_out,
        );
        
        let bind_group_alt = Self::create_bind_group(
            &device,
            &compute_pipeline,
            &uniform_buffer,
            &particle_buffer_out,
            &particle_buffer_in,
        );

        Ok(Self {
            device,
            queue,
            compute_pipeline,
            particle_buffer_in,
            particle_buffer_out,
            uniform_buffer,
            bind_group,
            bind_group_alt,
            grid_size: (width, height),
            tick: 0,
            ping_pong: 0,
            status: GpuStatus::Ready,
        })
    }
    
    /// Create bind group layout for compute shader
    fn create_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("bind_group_layout"),
                entries: &[
                    // Uniform buffer (binding 0)
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Input particle buffer (binding 1) - read-only
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Output particle buffer (binding 2) - read-write
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            },
        )
    }
    
    /// Create a bind group with specified buffers
    fn create_bind_group(
        device: &Device,
        pipeline: &wgpu::ComputePipeline,
        uniform: &Buffer,
        input: &Buffer,
        output: &Buffer,
    ) -> wgpu::BindGroup {
        let layout = pipeline.get_bind_group_layout(0);
        
        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("bind_group"),
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: input.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: output.as_entire_binding(),
                    },
                ],
            },
        )
    }

    /// Encode Grid state to GPU buffer (CPU → GPU)
    pub fn encode_grid(&mut self, grid_data: &[u32]) {
        // Write particle data to input buffer
        self.queue.write_buffer(
            &self.particle_buffer_in,
            0,
            bytemuck::cast_slice(grid_data),
        );
        
        // Write uniforms
        let uniforms = SimulationUniforms {
            width: self.grid_size.0,
            height: self.grid_size.1,
            tick: self.tick,
            _padding: 0,
        };
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );
    }

    /// Run one simulation tick on GPU and return command buffer
    pub fn tick(&mut self) -> CommandBuffer {
        // Create command encoder
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("compute_encoder"),
            },
        );
        
        // Select bind group based on ping-pong state
        let bind_group = if self.ping_pong == 0 {
            &self.bind_group
        } else {
            &self.bind_group_alt
        };
        
        // Run compute pass
        {
            let mut pass = encoder.begin_compute_pass(
                &wgpu::ComputePassDescriptor {
                    label: Some("particle_compute"),
                },
            );
            pass.set_pipeline(&self.compute_pipeline);
            pass.set_bind_group(0, bind_group, &[]);
            
            // Dispatch: one workgroup per 256 particles
            let cell_count = (self.grid_size.0 * self.grid_size.1) as u32;
            let workgroup_count = ((cell_count + 255) / 256).max(1);
            pass.dispatch(workgroup_count, 1, 1);
        }
        
        // Toggle ping-pong for next frame
        self.ping_pong = 1 - self.ping_pong;
        self.tick += 1;
        
        encoder.finish()
    }
    
    /// Submit GPU commands to queue
    pub fn submit(&mut self, commands: CommandBuffer) {
        self.queue.submit(std::iter::once(commands));
    }

    /// Get current status
    pub fn status(&self) -> &GpuStatus {
        &self.status
    }

    /// Get grid dimensions
    pub fn grid_size(&self) -> (u32, u32) {
        self.grid_size
    }
    
    /// Get current tick count
    pub fn tick_count(&self) -> u32 {
        self.tick
    }
    
    /// Get cell count
    pub fn cell_count(&self) -> usize {
        (self.grid_size.0 * self.grid_size.1) as usize
    }
}

// =============================================================================
// Grid Encoding/Decoding Utilities
// =============================================================================

/// Encode a Material to GPU u32 format
/// Bits 0-4: Material ID (0-12)
/// Bits 5-31: Reserved (0)
pub fn encode_material(material: Material) -> u32 {
    material as u32
}

/// Decode GPU u32 format to Material
pub fn decode_material(encoded: u32) -> Material {
    let id = encoded & 0x1F; // First 5 bits
    match id {
        0 => Material::Air,
        1 => Material::Sand,
        2 => Material::Water,
        3 => Material::Stone,
        4 => Material::Fire,
        5 => Material::Smoke,
        6 => Material::BlackHole,
        7 => Material::Steam,
        8 => Material::Ice,
        9 => Material::Oil,
        10 => Material::Wood,
        11 => Material::Lava,
        12 => Material::Ash,
        _ => Material::Air,
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

/// Decode GPU buffer to grid
#[cfg(feature = "gpu")]
pub fn decode_to_grid(data: &[u32], grid: &mut crate::Grid) {
    let size = grid.size();
    let mut idx = 0;
    
    for y in 0..size.height {
        for x in 0..size.width {
            if idx < data.len() {
                let material = decode_material(data[idx]);
                if material == Material::Air {
                    let _ = grid.remove(x, y);
                } else {
                    let _ = grid.set(x, y, crate::Particle::new(material));
                }
            }
            idx += 1;
        }
    }
}

/// GPU Simulation Runner - combines GPU pipeline with CPU fallback
pub struct GpuSimulator {
    /// GPU pipeline (if available)
    #[cfg(feature = "gpu")]
    pipeline: Option<GpuPipeline>,
    /// CPU fallback simulator
    cpu_sim: crate::Simulator,
    /// Grid size
    grid_size: (u32, u32),
    /// Use GPU flag
    gpu_enabled: bool,
}

impl GpuSimulator {
    /// Create new GPU simulator
    #[cfg(feature = "gpu")]
    pub async fn new(width: u32, height: u32) -> Self {
        let pipeline = GpuPipeline::new(width, height).await.ok();
        let gpu_enabled = pipeline.is_some();
        
        Self {
            pipeline,
            cpu_sim: crate::Simulator::new(),
            grid_size: (width, height),
            gpu_enabled,
        }
    }

    /// Create CPU-only simulator (no async)
    pub fn new_cpu(width: u32, height: u32) -> Self {
        Self {
            #[cfg(feature = "gpu")]
            pipeline: None,
            cpu_sim: crate::Simulator::new(),
            grid_size: (width, height),
            gpu_enabled: false,
        }
    }

    /// Run one simulation tick
    pub fn tick(&mut self, grid: &mut crate::Grid) {
        #[cfg(feature = "gpu")]
        {
            if let Some(ref mut pipeline) = self.pipeline {
                // Encode grid to GPU buffer
                let data = encode_grid(grid);
                pipeline.encode_grid(&data);
                
                // Run GPU compute
                let cmd = pipeline.tick();
                pipeline.submit(cmd);
                
                // Note: decode would need async buffer readback
                // For now, GPU simulation runs independently
                return;
            }
        }
        
        // CPU fallback
        self.cpu_sim.tick(grid);
    }

    /// Check if GPU is being used
    pub fn uses_gpu(&self) -> bool {
        self.gpu_enabled
    }

    /// Get GPU status
    #[cfg(feature = "gpu")]
    pub fn gpu_status(&self) -> Option<&GpuStatus> {
        self.pipeline.as_ref().map(|p| p.status())
    }
}

/// Simulation uniforms passed to compute shader (16-byte aligned)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct SimulationUniforms {
    /// Grid width
    width: u32,
    /// Grid height
    height: u32,
    /// Current tick count
    tick: u32,
    /// Padding for 16-byte alignment
    _padding: u32,
}

// =============================================================================
// CPU Fallback Implementation (when GPU feature not enabled)
// =============================================================================

/// Fallback CPU implementation when GPU not available
#[cfg(not(feature = "gpu"))]
pub struct GpuPipeline {
    status: GpuStatus,
    grid_size: (u32, u32),
    tick: u32,
}

#[cfg(not(feature = "gpu"))]
impl GpuPipeline {
    /// Create fallback pipeline (CPU-only)
    #[allow(dead_code)]
    pub fn new(_width: u32, _height: u32) -> Result<Self, String> {
        Ok(Self {
            status: GpuStatus::Unavailable("GPU feature not enabled".to_string()),
            grid_size: (0, 0),
            tick: 0,
        })
    }

    #[allow(dead_code)]
    pub fn encode_grid(&mut self, _grid: &[u32]) {
        // No-op: CPU grid already in memory
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
        self.tick += 1;
    }
    
    #[allow(dead_code)]
    pub fn submit(&mut self, _commands: ()) {
        // No-op
    }

    #[allow(dead_code)]
    pub fn status(&self) -> &GpuStatus {
        &self.status
    }

    #[allow(dead_code)]
    pub fn grid_size(&self) -> (u32, u32) {
        self.grid_size
    }
    
    #[allow(dead_code)]
    pub fn tick_count(&self) -> u32 {
        self.tick
    }
    
    #[allow(dead_code)]
    pub fn cell_count(&self) -> usize {
        (self.grid_size.0 * self.grid_size.1) as usize
    }
}

/// CPU-only GpuSimulator
pub struct GpuSimulator {
    cpu_sim: crate::Simulator,
    grid_size: (u32, u32),
    gpu_enabled: bool,
}

impl GpuSimulator {
    /// Create CPU-only simulator
    pub fn new_cpu(width: u32, height: u32) -> Self {
        Self {
            cpu_sim: crate::Simulator::new(),
            grid_size: (width, height),
            gpu_enabled: false,
        }
    }

    /// Create async GPU simulator (falls back to CPU if GPU unavailable)
    #[cfg(feature = "gpu")]
    pub async fn new(width: u32, height: u32) -> Self {
        Self::new_cpu(width, height) // Default to CPU for now
    }

    /// Run one simulation tick (CPU)
    pub fn tick(&mut self, grid: &mut crate::Grid) {
        self.cpu_sim.tick(grid);
    }

    /// GPU not available in CPU-only build
    pub fn uses_gpu(&self) -> bool {
        false
    }
}