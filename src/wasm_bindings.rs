//! WebAssembly bindings for the falling sand simulation
//! 
//! Provides JavaScript-friendly API for browser integration.

use wasm_bindgen::prelude::*;

/// WASM-exposed simulation state
#[wasm_bindgen]
pub struct Simulation {
    grid: crate::ChunkedGrid,
    simulator: crate::Simulator,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Simulation {
    /// Create a new simulation with given dimensions
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Simulation {
        console_error_panic_hook::set_once();
        
        let grid = crate::ChunkedGrid::new(crate::grid::GridSize::new(width, height));
        
        Simulation {
            grid,
            simulator: crate::Simulator::new(),
            width,
            height,
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
        // Re-create the grid
        self.grid = crate::ChunkedGrid::new(crate::grid::GridSize::new(self.width, self.height));
    }
    
    /// Render to RGBA pixel buffer
    /// Returns a Uint8Array of RGBA values (width * height * 4 bytes)
    pub fn render_rgba(&self) -> Vec<u8> {
        let mut pixels = Vec::with_capacity(self.width * self.height * 4);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.grid.get(x, y)
                    .map(|p| p.material.color())
                    .unwrap_or(crate::renderer::Color::rgb(20, 20, 30));
                
                pixels.push(color.r);
                pixels.push(color.g);
                pixels.push(color.b);
                pixels.push(color.a);
            }
        }
        
        pixels
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
