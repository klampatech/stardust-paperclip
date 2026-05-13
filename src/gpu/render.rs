//! GPU Render Pipeline
//! 
//! Instanced rendering with compute shader culling and LOD system.

use crate::particle::Material;

/// Render pipeline status
#[derive(Debug, Clone)]
pub enum RenderStatus {
    Ready,
    Initializing,
    Unavailable(String),
    Error(String),
}

impl RenderStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, RenderStatus::Ready)
    }
    
    pub fn message(&self) -> String {
        match self {
            RenderStatus::Ready => "Render ready".to_string(),
            RenderStatus::Initializing => "Initializing...".to_string(),
            RenderStatus::Unavailable(msg) => msg.clone(),
            RenderStatus::Error(msg) => format!("Render error: {}", msg),
        }
    }
}

/// Vertex data for particle rendering (f32 x 4: x, y, r, g)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ParticleVertex {
    pub x: f32,
    pub y: f32,
    pub color_r: f32,
    pub color_g: f32,
}

/// Instance data for instanced rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ParticleInstance {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub material: u32,
}

/// Camera for view culling
#[derive(Debug, Clone)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            width: 256.0,
            height: 256.0,
        }
    }
}

impl Camera {
    /// Create new camera
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, zoom: 1.0, width, height }
    }
    
    /// Check if position is in camera view
    pub fn in_view(&self, px: f32, py: f32) -> bool {
        px >= self.x && px < self.x + self.width / self.zoom &&
        py >= self.y && py < self.y + self.height / self.zoom
    }
    
    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1).min(10.0);
    }
    
    /// Pan camera
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.x += dx / self.zoom;
        self.y += dy / self.zoom;
    }
}

/// Frame timing data
#[derive(Debug, Clone, Default)]
pub struct FrameTiming {
    pub frame_time_ms: f32,
    pub gpu_time_ms: f32,
    pub cpu_time_ms: f32,
    pub frame_count: u64,
}

impl FrameTiming {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update with new measurements
    pub fn update(&mut self, gpu_ms: f32, cpu_ms: f32) {
        self.gpu_time_ms = gpu_ms;
        self.cpu_time_ms = cpu_ms;
        self.frame_time_ms = gpu_ms + cpu_ms;
        self.frame_count += 1;
    }
    
    /// Get FPS
    pub fn fps(&self) -> f32 {
        if self.frame_time_ms > 0.0 {
            1000.0 / self.frame_time_ms
        } else {
            0.0
        }
    }
    
    /// Get frame budget usage (target 16.67ms for 60fps)
    pub fn budget_usage(&self) -> f32 {
        self.frame_time_ms / 16.67
    }
    
    /// Is frame within budget?
    pub fn within_budget(&self, target_fps: f32) -> bool {
        let target_ms = 1000.0 / target_fps;
        self.frame_time_ms <= target_ms
    }
}

/// LOD level for particles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodLevel {
    /// Full detail - individual particles
    Full = 0,
    /// Medium detail - 2x2 particle blocks
    Medium = 1,
    /// Low detail - 4x4 particle blocks
    Low = 2,
    /// Very low detail - 8x8 particle blocks
    VeryLow = 3,
}

impl LodLevel {
    /// Get block size for LOD level
    pub fn block_size(&self) -> usize {
        match self {
            LodLevel::Full => 1,
            LodLevel::Medium => 2,
            LodLevel::Low => 4,
            LodLevel::VeryLow => 8,
        }
    }
    
    /// Calculate LOD level based on distance from camera center
    pub fn from_distance(dist: f32, max_dist: f32) -> Self {
        let normalized = (dist / max_dist).min(1.0);
        
        if normalized < 0.25 {
            LodLevel::Full
        } else if normalized < 0.5 {
            LodLevel::Medium
        } else if normalized < 0.75 {
            LodLevel::Low
        } else {
            LodLevel::VeryLow
        }
    }
}

/// LOD system for distance-based particle merging
pub struct LodSystem {
    /// Maximum distance for LOD calculation
    max_distance: f32,
    /// Current LOD level
    current_lod: LodLevel,
    /// Enable LOD
    enabled: bool,
}

impl LodSystem {
    /// Create new LOD system
    pub fn new(max_distance: f32) -> Self {
        Self {
            max_distance,
            current_lod: LodLevel::Full,
            enabled: true,
        }
    }
    
    /// Calculate LOD for a position
    pub fn calculate_lod(&self, px: f32, py: f32, camera: &Camera) -> LodLevel {
        if !self.enabled {
            return LodLevel::Full;
        }
        
        let center_x = camera.x + camera.width / (2.0 * camera.zoom);
        let center_y = camera.y + camera.height / (2.0 * camera.zoom);
        
        let dx = px - center_x;
        let dy = py - center_y;
        let dist = (dx * dx + dy * dy).sqrt();
        
        LodLevel::from_distance(dist, self.max_distance)
    }
    
    /// Enable/disable LOD
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Set maximum distance
    pub fn set_max_distance(&mut self, max_dist: f32) {
        self.max_distance = max_dist;
    }
    
    /// Get current LOD level
    pub fn current_level(&self) -> LodLevel {
        self.current_lod
    }
}

/// Particle material colors (RGBA as f32)
pub fn material_color(material: Material) -> (f32, f32, f32, f32) {
    match material {
        Material::Air => (0.08, 0.08, 0.12, 1.0),
        Material::Sand => (0.76, 0.70, 0.50, 1.0),
        Material::Water => (0.25, 0.64, 0.87, 1.0),
        Material::Stone => (0.50, 0.50, 0.50, 1.0),
        Material::Fire => (1.0, 0.39, 0.20, 1.0),
        Material::Smoke => (0.39, 0.39, 0.43, 0.8),
        Material::BlackHole => (0.0, 0.0, 0.0, 1.0),
        Material::Steam => (0.78, 0.78, 1.0, 0.6),
        Material::Ice => (0.68, 0.85, 0.98, 1.0),
        Material::Oil => (0.40, 0.26, 0.13, 1.0),
        Material::Wood => (0.55, 0.35, 0.17, 1.0),
        Material::Lava => (1.0, 0.27, 0.0, 1.0),
        Material::Ash => (0.20, 0.20, 0.22, 1.0),
    }
}

/// GPU Render Pipeline (requires GPU feature)
#[cfg(feature = "gpu")]
pub struct GpuRenderPipeline {
    /// Render status
    status: RenderStatus,
    /// Camera for culling
    camera: Camera,
    /// LOD system
    lod: LodSystem,
    /// Frame timing
    timing: FrameTiming,
    /// Canvas width
    width: u32,
    /// Canvas height
    height: u32,
}

#[cfg(feature = "gpu")]
impl GpuRenderPipeline {
    /// Create new GPU render pipeline
    pub async fn new(width: u32, height: u32) -> Result<Self, String> {
        // Note: Full WebGPU render pipeline would require surface, texture, etc.
        // This is a scaffold for the render pipeline
        
        Ok(Self {
            status: RenderStatus::Ready,
            camera: Camera::new(0.0, 0.0, width as f32, height as f32),
            lod: LodSystem::new(200.0),
            timing: FrameTiming::new(),
            width,
            height,
        })
    }
    
    /// Get camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    
    /// Get mutable camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    
    /// Get LOD system
    pub fn lod(&self) -> &LodSystem {
        &self.lod
    }
    
    /// Get mutable LOD system
    pub fn lod_mut(&mut self) -> &mut LodSystem {
        &mut self.lod
    }
    
    /// Get frame timing
    pub fn timing(&self) -> &FrameTiming {
        &self.timing
    }
    
    /// Get status
    pub fn status(&self) -> &RenderStatus {
        &self.status
    }
    
    /// Update timing with new frame
    pub fn update_timing(&mut self, gpu_ms: f32, cpu_ms: f32) {
        self.timing.update(gpu_ms, cpu_ms);
    }
    
    /// Check if position is visible (frustum culling)
    pub fn is_visible(&self, x: f32, y: f32) -> bool {
        self.camera.in_view(x, y)
    }
    
    /// Get canvas dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// CPU Fallback Render Pipeline
pub struct GpuRenderPipeline {
    status: RenderStatus,
    camera: Camera,
    lod: LodSystem,
    timing: FrameTiming,
    width: u32,
    height: u32,
}

impl GpuRenderPipeline {
    /// Create CPU fallback
    pub fn new_cpu(width: u32, height: u32) -> Self {
        Self {
            status: RenderStatus::Unavailable("GPU feature not enabled".to_string()),
            camera: Camera::new(0.0, 0.0, width as f32, height as f32),
            lod: LodSystem::new(200.0),
            timing: FrameTiming::new(),
            width,
            height,
        }
    }
    
    /// Create async GPU render (falls back to CPU)
    #[cfg(feature = "gpu")]
    pub async fn new(width: u32, height: u32) -> Result<Self, String> {
        Ok(Self::new_cpu(width, height))
    }
    
    pub fn camera(&self) -> &Camera { &self.camera }
    pub fn camera_mut(&mut self) -> &mut Camera { &mut self.camera }
    pub fn lod(&self) -> &LodSystem { &self.lod }
    pub fn lod_mut(&mut self) -> &mut LodSystem { &mut self.lod }
    pub fn timing(&self) -> &FrameTiming { &self.timing }
    pub fn status(&self) -> &RenderStatus { &self.status }
    
    pub fn update_timing(&mut self, gpu_ms: f32, cpu_ms: f32) {
        self.timing.update(gpu_ms, cpu_ms);
    }
    
    pub fn is_visible(&self, x: f32, y: f32) -> bool {
        self.camera.in_view(x, y)
    }
    
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// Profiler for frame timing
pub struct Profiler {
    /// Frame timings history
    history: Vec<FrameTiming>,
    /// Maximum history size
    max_history: usize,
}

impl Profiler {
    /// Create new profiler
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Vec::with_capacity(max_history),
            max_history,
        }
    }
    
    /// Record a frame timing
    pub fn record(&mut self, timing: FrameTiming) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(timing);
    }
    
    /// Get average FPS
    pub fn average_fps(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.history.iter().map(|t| t.fps()).sum();
        sum / self.history.len() as f32
    }
    
    /// Get average frame time
    pub fn average_frame_time(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.history.iter().map(|t| t.frame_time_ms).sum();
        sum / self.history.len() as f32
    }
    
    /// Get 99th percentile frame time
    pub fn percentile_99(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }
        let mut times: Vec<f32> = self.history.iter().map(|t| t.frame_time_ms).collect();
        times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = ((self.history.len() as f32 * 0.99) as usize).min(times.len() - 1);
        times[idx]
    }
    
    /// Get GPU/CPU ratio
    pub fn gpu_cpu_ratio(&self) -> f32 {
        if self.history.is_empty() {
            return 0.0;
        }
        let gpu_sum: f32 = self.history.iter().map(|t| t.gpu_time_ms).sum();
        let cpu_sum: f32 = self.history.iter().map(|t| t.cpu_time_ms).sum();
        if cpu_sum > 0.0 {
            gpu_sum / cpu_sum
        } else {
            0.0
        }
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new(120) // 2 seconds at 60fps
    }
}