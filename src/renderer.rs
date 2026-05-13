//! Canvas2D rendering pipeline for the falling sand simulation
//! 
//! Provides pixel-based rendering optimized for particle grids.

use crate::grid::Grid;
use crate::particle::{Material, Particle};

/// RGBA color representation
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// Material colors for rendering
impl Material {
    pub fn color(&self) -> Color {
        match self {
            Material::Air => Color::rgb(20, 20, 30),       // Dark background
            Material::Sand => Color::rgb(194, 178, 128),  // Sandy beige
            Material::Water => Color::rgb(64, 164, 223),   // Blue
            Material::Stone => Color::rgb(128, 128, 128), // Gray
            Material::Fire => Color::rgb(255, 100, 50),    // Orange-red
            Material::Smoke => Color::rgb(100, 100, 110), // Dark gray
            Material::BlackHole => Color::rgb(0, 0, 0),    // Pure black
            Material::Steam => Color::rgb(200, 200, 255),  // Light blue
            Material::Ice => Color::rgb(173, 216, 250),    // Ice blue
            Material::Oil => Color::rgb(101, 67, 33),     // Dark brown
            Material::Wood => Color::rgb(139, 90, 43),     // Brown
            Material::Lava => Color::rgb(255, 69, 0),     // Red-orange
            Material::Ash => Color::rgb(50, 50, 55),      // Dark gray (burned)
        }
    }
    
    /// Color variation based on particle properties
    pub fn render_color(&self, particle: &Particle) -> Color {
        let base = self.color();
        
        match self {
            Material::Fire => {
                // Fire flickers between orange and yellow
                let intensity = (particle.lifetime % 10) as u8;
                let r = 255.min(base.r + intensity * 2);
                let g = (100 + intensity * 5).min(255);
                Color::rgb(r, g, base.b)
            }
            Material::Smoke => {
                // Smoke varies in darkness
                let dark = (100 + (particle.lifetime % 20)) as u8;
                Color::rgb(dark, dark, dark + 10)
            }
            Material::Lava => {
                // Lava glows based on temperature
                let temp = particle.temperature.min(1500.0) / 1500.0;
                let r = 255;
                let g = (69.0 + temp * 100.0) as u8;
                let b = 0;
                Color::rgb(r, g.min(255), b)
            }
            Material::Steam => {
                // Steam varies in opacity based on lifetime
                let alpha = ((100 - particle.lifetime.min(100)) as f32 * 2.55) as u8;
                Color::new(base.r, base.g, base.b, alpha.max(50))
            }
            Material::Ice => {
                // Ice has slight sparkle effect
                let sparkle = if rand_u8() % 20 == 0 { 30 } else { 0 };
                Color::new(
                    (base.r + sparkle).min(255),
                    (base.g + sparkle).min(255),
                    (base.b + sparkle).min(255),
                    255
                )
            }
            _ => base,
        }
    }
}

/// Canvas2D renderer for the simulation
pub struct Renderer {
    /// Pixel buffer (RGBA)
    pixels: Vec<u8>,
    /// Canvas width
    width: usize,
    /// Canvas height  
    height: usize,
    /// Scale factor (each cell = scale x scale pixels)
    scale: usize,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(width: usize, height: usize, scale: usize) -> Self {
        let pixel_count = width * height * scale * scale * 4; // RGBA
        Self {
            pixels: vec![0; pixel_count],
            width,
            height,
            scale,
        }
    }
    
    /// Get pixel buffer for direct access
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
    
    /// Render a grid to the pixel buffer
    pub fn render(&mut self, grid: &Grid) {
        let scale = self.scale;
        
        for y in 0..self.height {
            for x in 0..self.width {
                let particle = grid.get(x, y).unwrap_or(Particle::empty());
                let color = particle.material.render_color(&particle);
                
                // Fill scaled pixel block
                self.fill_block(x, y, scale, color);
            }
        }
    }
    
    /// Fill a scaled block with a color
    fn fill_block(&mut self, x: usize, y: usize, scale: usize, color: Color) {
        for dy in 0..scale {
            for dx in 0..scale {
                let px = x * scale + dx;
                let py = y * scale + dy;
                
                if px < self.width * scale && py < self.height * scale {
                    let idx = (py * self.width * scale + px) * 4;
                    self.pixels[idx] = color.r;
                    self.pixels[idx + 1] = color.g;
                    self.pixels[idx + 2] = color.b;
                    self.pixels[idx + 3] = color.a;
                }
            }
        }
    }
    
    /// Clear the canvas
    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }
    
    /// Get dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width * self.scale, self.height * self.scale)
    }
}

/// ANSI terminal renderer for ASCII output
pub struct TerminalRenderer {
    width: usize,
    height: usize,
}

impl TerminalRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    
    /// Render grid to string for terminal display
    pub fn render(&self, grid: &Grid) -> String {
        let mut output = format!("+{}+\n", "-".repeat(self.width));
        
        for y in 0..self.height {
            output.push('|');
            for x in 0..self.width {
                let ch = grid.get(x, y)
                    .map(|p| self.material_char(p.material))
                    .unwrap_or('?');
                output.push(ch);
            }
            output.push_str("|\n");
        }
        
        output.push_str(&format!("+{}+", "-".repeat(self.width)));
        output
    }
    
    fn material_char(&self, material: Material) -> char {
        match material {
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
        }
    }
}

/// Simple pseudo-random for renderer effects
fn rand_u8() -> u8 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u8)
        .unwrap_or(42)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GridSize;
    
    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }
    
    #[test]
    fn test_material_colors() {
        assert_eq!(Material::Sand.color().r, 194);
        assert_eq!(Material::Water.color().b, 223);
        assert_eq!(Material::Fire.color().r, 255);
    }
    
    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(64, 64, 4);
        assert_eq!(renderer.pixels.len(), 64 * 64 * 4 * 4 * 4); // width * height * scale^2 * RGBA
    }
    
    #[test]
    fn test_render_grid() {
        let mut grid = Grid::new(GridSize::new(8, 8));
        grid.spawn(4, 4, Material::Sand);
        
        let mut renderer = Renderer::new(8, 8, 1);
        renderer.render(&grid);
        
        // Check pixel was set
        let idx = (4 * 8 + 4) * 4;
        assert_eq!(renderer.pixels[idx], 194); // Sand r
        assert_eq!(renderer.pixels[idx + 1], 178); // Sand g
        assert_eq!(renderer.pixels[idx + 2], 128); // Sand b
    }
    
}
