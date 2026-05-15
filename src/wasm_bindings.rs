//! WebAssembly bindings for the falling sand simulation
//! 
//! Provides JavaScript-friendly API for browser integration.
//! 
//! # Usage
//! ```javascript
//! import init, { Simulation } from './pkg/falling_sand.js';
//! 
//! await init();
//! const sim = new Simulation(200, 200);
//! sim.spawn(100, 100, 'sand');
//! sim.tick();
//! const pixels = sim.render_rgba();
//! ```
//! 
//! # WASM Integration Notes
//! - Uses ChunkedGrid for optimized 50,000+ particle simulations
//! - Pre-allocated pixel buffers for zero-allocation rendering
//! - Circular brush spawning for efficient particle placement

use wasm_bindgen::prelude::*;

/// WASM-exposed simulation state
#[wasm_bindgen]
pub struct Simulation {
    grid: crate::chunk::ChunkedGrid,
    simulator: crate::simulation::Simulator,
    width: usize,
    height: usize,
    /// Cache for pixel buffer to avoid allocation each frame
    pixel_cache: Vec<u8>,
}

impl Simulation {
    /// Get a pre-allocated pixel buffer for rendering
    fn get_pixel_buffer(&mut self) -> &mut Vec<u8> {
        let needed = self.width * self.height * 4;
        if self.pixel_cache.len() != needed {
            self.pixel_cache.resize(needed, 0);
        }
        &mut self.pixel_cache
    }
}

#[wasm_bindgen]
impl Simulation {
    /// Create a new simulation with given dimensions
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Simulation {
        console_error_panic_hook::set_once();
        
        // Clamp dimensions to reasonable limits
        let width = width.min(1000);
        let height = height.min(1000);
        
        let grid = crate::chunk::ChunkedGrid::new(crate::grid::GridSize::new(width, height));
        
        let pixel_cache = vec![0u8; width * height * 4];
        
        Simulation {
            grid,
            simulator: crate::simulation::Simulator::new(),
            width,
            height,
            pixel_cache,
        }
    }
    
    /// Run one simulation tick
    pub fn tick(&mut self) {
        self.simulator.tick_chunked(&mut self.grid);
    }
    
    /// Run multiple simulation ticks
    pub fn tick_many(&mut self, count: usize) {
        for _ in 0..count {
            self.simulator.tick_chunked(&mut self.grid);
        }
    }
    
    /// Spawn a particle at position
    pub fn spawn(&mut self, x: usize, y: usize, material: &str) -> bool {
        let mat = match material {
            "sand" => crate::particle::Material::Sand,
            "water" => crate::particle::Material::Water,
            "fire" => crate::particle::Material::Fire,
            "smoke" => crate::particle::Material::Smoke,
            "stone" => crate::particle::Material::Stone,
            "oil" => crate::particle::Material::Oil,
            "lava" => crate::particle::Material::Lava,
            "ice" => crate::particle::Material::Ice,
            "wood" => crate::particle::Material::Wood,
            "ash" => crate::particle::Material::Ash,
            "steam" => crate::particle::Material::Steam,
            "blackhole" => crate::particle::Material::BlackHole,
            _ => return false,
        };
        
        self.grid.spawn(x, y, mat)
    }
    
    /// Get particle material at position
    pub fn get_material(&self, x: usize, y: usize) -> String {
        self.grid.get(x, y)
            .map(|p| match p.material {
                crate::particle::Material::Air => "air",
                crate::particle::Material::Sand => "sand",
                crate::particle::Material::Water => "water",
                crate::particle::Material::Stone => "stone",
                crate::particle::Material::Fire => "fire",
                crate::particle::Material::Smoke => "smoke",
                crate::particle::Material::BlackHole => "blackhole",
                crate::particle::Material::Steam => "steam",
                crate::particle::Material::Ice => "ice",
                crate::particle::Material::Oil => "oil",
                crate::particle::Material::Wood => "wood",
                crate::particle::Material::Lava => "lava",
                crate::particle::Material::Ash => "ash",
            })
            .unwrap_or("air")
            .to_string()
    }
    
    /// Get particle lifetime (for fire/smoke rendering)
    pub fn get_lifetime(&self, x: usize, y: usize) -> u32 {
        self.grid.get(x, y)
            .map(|p| p.lifetime)
            .unwrap_or(0)
    }
    
    /// Get total particle count
    pub fn particle_count(&self) -> usize {
        self.grid.total_particles()
    }
    
    /// Get grid width
    pub fn width(&self) -> usize {
        self.width
    }
    
    /// Get grid height
    pub fn height(&self) -> usize {
        self.height
    }
    
    /// Clear the grid
    pub fn clear(&mut self) {
        // Re-create the grid with same dimensions
        self.grid = crate::chunk::ChunkedGrid::new(crate::grid::GridSize::new(self.width, self.height));
    }
    
    /// Spawn multiple particles in a circular brush pattern
    /// Returns the number of particles successfully spawned
    pub fn spawn_brush(&mut self, cx: usize, cy: usize, material: &str, radius: usize) -> usize {
        let mat = match material {
            "sand" => crate::particle::Material::Sand,
            "water" => crate::particle::Material::Water,
            "fire" => crate::particle::Material::Fire,
            "smoke" => crate::particle::Material::Smoke,
            "stone" => crate::particle::Material::Stone,
            "oil" => crate::particle::Material::Oil,
            "lava" => crate::particle::Material::Lava,
            "ice" => crate::particle::Material::Ice,
            "wood" => crate::particle::Material::Wood,
            "ash" => crate::particle::Material::Ash,
            "steam" => crate::particle::Material::Steam,
            "blackhole" => crate::particle::Material::BlackHole,
            _ => return 0,
        };
        
        let mut count = 0;
        let radius = radius.min(50); // Limit brush size
        
        for dy in 0..=radius * 2 {
            for dx in 0..=radius * 2 {
                let dist_sq = (dx as i32 - radius as i32).pow(2) + (dy as i32 - radius as i32).pow(2);
                if dist_sq <= (radius as i32).pow(2) {
                    let x = cx.saturating_add(dx).saturating_sub(radius);
                    let y = cy.saturating_add(dy).saturating_sub(radius);
                    if self.grid.spawn(x, y, mat) {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }
    
    /// Render to RGBA pixel buffer
    /// Returns a Uint8Array of RGBA values (width * height * 4 bytes)
    /// Uses pre-allocated buffer for performance
    pub fn render_rgba(&mut self) -> Vec<u8> {
        let pixels = self.get_pixel_buffer();
        let bg_color = crate::renderer::Color::rgb(20, 20, 30);
        
        // Fill with background
        for chunk in pixels.chunks_exact_mut(4) {
            chunk[0] = bg_color.r;
            chunk[1] = bg_color.g;
            chunk[2] = bg_color.b;
            chunk[3] = bg_color.a;
        }
        
        // Render particles using render_color for effects
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(p) = self.grid.get(x, y) {
                    if p.material != crate::particle::Material::Air {
                        let color = p.material.render_color(&p);
                        let idx = (y * self.width + x) * 4;
                        pixels[idx] = color.r;
                        pixels[idx + 1] = color.g;
                        pixels[idx + 2] = color.b;
                        pixels[idx + 3] = color.a;
                    }
                }
            }
        }
        
        pixels.clone()
    }
    
    /// Render directly into a pre-allocated buffer (zero-copy for WASM)
    /// Call this followed by `take_pixel_buffer()` for optimal performance
    pub fn render_rgba_direct(&mut self) {
        let pixels = self.get_pixel_buffer();
        let bg_color = crate::renderer::Color::rgb(20, 20, 30);
        
        // Fill with background
        for chunk in pixels.chunks_exact_mut(4) {
            chunk[0] = bg_color.r;
            chunk[1] = bg_color.g;
            chunk[2] = bg_color.b;
            chunk[3] = bg_color.a;
        }
        
        // Render particles using render_color for effects
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(p) = self.grid.get(x, y) {
                    if p.material != crate::particle::Material::Air {
                        let color = p.material.render_color(&p);
                        let idx = (y * self.width + x) * 4;
                        pixels[idx] = color.r;
                        pixels[idx + 1] = color.g;
                        pixels[idx + 2] = color.b;
                        pixels[idx + 3] = color.a;
                    }
                }
            }
        }
    }
    
    /// Take ownership of the pixel buffer (for zero-copy WASM transfer)
    pub fn take_pixel_buffer(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.pixel_cache)
    }
}

/// Color helper for WASM
#[wasm_bindgen]
impl crate::renderer::Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> crate::renderer::Color {
        crate::renderer::Color { r, g, b, a }
    }
}

/// Export panic hook for better error messages in WASM
mod console_error_panic_hook {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    
    pub fn set_once() {
        SET_HOOK.call_once(|| {
            std::panic::set_hook(Box::new(|info| {
                web_sys::console::error_1(&format!("Rust panic: {:?}", info).into());
            }));
        });
    }
}

// =============================================================================
// Space Game Objects - WASM bindings for high-level game entities
// =============================================================================

/// Unique identifier for game objects
#[wasm_bindgen]
pub struct GameObjectId(pub u64);

#[wasm_bindgen]
impl GameObjectId {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameObjectId {
        GameObjectId(super::game_objects::GameObjectId::new().0)
    }
}

/// Types of space game objects
#[wasm_bindgen]
pub enum GameObjectType {
    Celestial,
    Star,
    Spacecraft,
    Station,
    Comet,
    Debris,
    Nebula,
}

impl From<GameObjectType> for super::game_objects::GameObjectType {
    fn from(t: GameObjectType) -> Self {
        match t {
            GameObjectType::Celestial => super::game_objects::GameObjectType::Celestial,
            GameObjectType::Star => super::game_objects::GameObjectType::Star,
            GameObjectType::Spacecraft => super::game_objects::GameObjectType::Spacecraft,
            GameObjectType::Station => super::game_objects::GameObjectType::Station,
            GameObjectType::Comet => super::game_objects::GameObjectType::Comet,
            GameObjectType::Debris => super::game_objects::GameObjectType::Debris,
            GameObjectType::Nebula => super::game_objects::GameObjectType::Nebula,
        }
    }
}

impl From<super::game_objects::GameObjectType> for GameObjectType {
    fn from(t: super::game_objects::GameObjectType) -> Self {
        match t {
            super::game_objects::GameObjectType::Celestial => GameObjectType::Celestial,
            super::game_objects::GameObjectType::Star => GameObjectType::Star,
            super::game_objects::GameObjectType::Spacecraft => GameObjectType::Spacecraft,
            super::game_objects::GameObjectType::Station => GameObjectType::Station,
            super::game_objects::GameObjectType::Comet => GameObjectType::Comet,
            super::game_objects::GameObjectType::Debris => GameObjectType::Debris,
            super::game_objects::GameObjectType::Nebula => GameObjectType::Nebula,
        }
    }
}

/// Position in 2D space
#[wasm_bindgen]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
    
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl From<super::game_objects::Position> for Position {
    fn from(p: super::game_objects::Position) -> Self {
        Position { x: p.x, y: p.y }
    }
}

/// Velocity in 2D space
#[wasm_bindgen]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Velocity {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl From<super::game_objects::Velocity> for Velocity {
    fn from(v: super::game_objects::Velocity) -> Self {
        Velocity { x: v.x, y: v.y }
    }
}

/// Manager for all game objects (stars, planets, spacecraft, etc.)
#[wasm_bindgen]
pub struct GameObjectManager {
    manager: super::game_objects::GameObjectManager,
}

#[wasm_bindgen]
impl GameObjectManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameObjectManager {
        GameObjectManager {
            manager: super::game_objects::GameObjectManager::new(),
        }
    }
    
    /// Get number of active game objects
    pub fn count(&self) -> usize {
        self.manager.active_objects().len()
    }
    
    /// Get all active object types as strings
    pub fn list_types(&self) -> String {
        let types: Vec<String> = self.manager.active_objects()
            .iter()
            .map(|o| format!("{:?}", o.object_type))
            .collect();
        types.join(",")
    }
    
    /// Get positions of all active objects as JSON
    pub fn get_positions_json(&self) -> String {
        let positions: Vec<String> = self.manager.active_objects()
            .iter()
            .map(|o| format!("{{\"x\":{},\"y\":{}}}", o.position.x, o.position.y))
            .collect();
        format!("[{}]", positions.join(","))
    }
}