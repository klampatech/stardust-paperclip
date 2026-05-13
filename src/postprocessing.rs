//! Post-processing pipeline for visual effects
//! 
//! Implements bloom, motion blur, color grading, and other post-processing effects.

use crate::renderer::Color;

/// Post-processing configuration
#[derive(Debug, Clone)]
pub struct PostProcessingConfig {
    /// Enable bloom effect (glow around bright particles)
    pub bloom_enabled: bool,
    /// Bloom intensity (0.0 - 1.0)
    pub bloom_intensity: f32,
    /// Bloom threshold - minimum brightness to bloom
    pub bloom_threshold: f32,
    /// Bloom radius (number of blur iterations)
    pub bloom_radius: u8,
    
    /// Enable motion blur effect
    pub motion_blur_enabled: bool,
    /// Motion blur intensity (0.0 - 1.0)
    pub motion_blur_intensity: f32,
    
    /// Enable screen shake effect
    pub screen_shake_enabled: bool,
    /// Screen shake decay rate
    pub screen_shake_decay: f32,
    
    /// Enable color grading
    pub color_grading_enabled: bool,
    /// Color grading mode
    pub color_grading_mode: ColorGradingMode,
    
    /// Enable vignette effect
    pub vignette_enabled: bool,
    /// Vignette intensity
    pub vignette_intensity: f32,
    /// Vignette radius (0.0 - 1.0, how much of the screen is affected)
    pub vignette_radius: f32,
    
    /// Enable scanlines (retro effect)
    pub scanlines_enabled: bool,
    /// Scanline intensity
    pub scanline_intensity: f32,
    
    /// Enable chromatic aberration
    pub chromatic_aberration_enabled: bool,
    /// Chromatic aberration strength
    pub chromatic_strength: f32,
}

impl Default for PostProcessingConfig {
    fn default() -> Self {
        Self {
            // Bloom - good for fire and lava
            bloom_enabled: true,
            bloom_intensity: 0.6,
            bloom_threshold: 0.7,
            bloom_radius: 3,
            
            // Motion blur - subtle effect
            motion_blur_enabled: false,
            motion_blur_intensity: 0.3,
            
            // Screen shake - for explosions and black hole events
            screen_shake_enabled: true,
            screen_shake_decay: 0.9,
            
            // Color grading
            color_grading_enabled: true,
            color_grading_mode: ColorGradingMode::Vibrant,
            
            // Vignette
            vignette_enabled: true,
            vignette_intensity: 0.4,
            vignette_radius: 0.8,
            
            // Scanlines
            scanlines_enabled: false,
            scanline_intensity: 0.1,
            
            // Chromatic aberration
            chromatic_aberration_enabled: false,
            chromatic_strength: 0.5,
        }
    }
}

/// Color grading presets
#[derive(Debug, Clone, Copy)]
pub enum ColorGradingMode {
    /// Default colors, no grading
    None,
    /// Slightly saturated, warm highlights
    Vibrant,
    /// Cooler tones, blue shadows
    Cool,
    /// Warm tones, orange highlights
    Warm,
    /// Desaturated, film-like
    Cinematic,
    /// High contrast, punchy colors
    HighContrast,
    /// Retro 8-bit style
    Retro,
}

impl ColorGradingMode {
    /// Apply color grading to a pixel
    pub fn apply(&self, color: &mut Color) {
        match self {
            ColorGradingMode::None => {}
            ColorGradingMode::Vibrant => {
                // Slightly boost saturation and warmth
                let avg = ((color.r as u32 + color.g as u32 + color.b as u32) / 3) as u8;
                let boost = |c: u8| -> u8 {
                    let diff = (c as i16 - avg as i16) * 3 / 2;
                    (c as i16 + diff).max(0).min(255) as u8
                };
                color.r = boost(color.r);
                color.g = (color.g * 105 / 100).min(255);
                color.b = (color.b * 95 / 100).min(255);
            }
            ColorGradingMode::Cool => {
                // Shift toward blue
                color.r = (color.r as f32 * 0.9) as u8;
                color.g = (color.g as f32 * 0.95) as u8;
                color.b = (color.b as f32 * 1.1).min(255.0) as u8;
            }
            ColorGradingMode::Warm => {
                // Shift toward orange/red
                color.r = (color.r as f32 * 1.1).min(255.0) as u8;
                color.g = (color.g as f32 * 1.05).min(255.0) as u8;
                color.b = (color.b as f32 * 0.9) as u8;
            }
            ColorGradingMode::Cinematic => {
                // Desaturate and add contrast
                let avg = ((color.r as u32 + color.g as u32 + color.b as u32) / 3) as u8;
                let contrast = |c: u8| -> u8 {
                    let diff = c as i16 - 128;
                    let new_val = 128 + (diff * 12 / 10);
                    new_val.max(0).min(255) as u8
                };
                // Blend 30% toward desaturated
                let desaturated = avg;
                color.r = ((color.r as u32 * 7 + desaturated as u32 * 3) / 10) as u8;
                color.g = ((color.g as u32 * 7 + desaturated as u32 * 3) / 10) as u8;
                color.b = ((color.b as u32 * 7 + desaturated as u32 * 3) / 10) as u8;
            }
            ColorGradingMode::HighContrast => {
                // Boost contrast significantly
                let contrast = |c: u8| -> u8 {
                    let diff = c as i32 - 128;
                    let new_val = 128 + (diff * 15 / 10);
                    new_val.max(0).min(255) as u8
                };
                color.r = contrast(color.r);
                color.g = contrast(color.g);
                color.b = contrast(color.b);
            }
            ColorGradingMode::Retro => {
                // Reduce to 16-color palette style
                color.r = (color.r / 16) * 16;
                color.g = (color.g / 16) * 16;
                color.b = (color.b / 16) * 16;
            }
        }
    }
}

/// Screen shake state
#[derive(Debug, Clone)]
pub struct ScreenShake {
    /// Current shake intensity (0.0 - 1.0)
    pub intensity: f32,
    /// Shake offset X
    pub offset_x: f32,
    /// Shake offset Y
    pub offset_y: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl ScreenShake {
    /// Trigger a new screen shake
    pub fn trigger(&mut self, intensity: f32) {
        self.intensity = self.intensity.max(intensity);
    }
    
    /// Update shake state (call each frame)
    pub fn update(&mut self, decay: f32) {
        if self.intensity > 0.01 {
            // Generate random offset based on intensity
            use std::time::{SystemTime, UNIX_EPOCH};
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(42);
            
            let magnitude = self.intensity * 10.0;
            self.offset_x = ((time as f32 % 100.0) - 50.0) / 50.0 * magnitude;
            self.offset_y = (((time >> 8) as f32 % 100.0) - 50.0) / 50.0 * magnitude;
            
            // Decay intensity
            self.intensity *= decay;
        } else {
            self.intensity = 0.0;
            self.offset_x = 0.0;
            self.offset_y = 0.0;
        }
    }
}

/// Post-processing processor
pub struct PostProcessor {
    /// Configuration
    config: PostProcessingConfig,
    /// Screen shake state
    screen_shake: ScreenShake,
    /// Bloom buffer (half-res for performance)
    bloom_buffer: Vec<u8>,
    /// Temp buffer for blur passes
    temp_buffer: Vec<u8>,
}

impl PostProcessor {
    /// Create a new post-processor
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            config: PostProcessingConfig::default(),
            screen_shake: ScreenShake::default(),
            bloom_buffer: vec![0; width * height * 4],
            temp_buffer: vec![0; width * height * 4],
        }
    }
    
    /// Create with custom config
    pub fn with_config(width: usize, height: usize, config: PostProcessingConfig) -> Self {
        Self {
            config,
            screen_shake: ScreenShake::default(),
            bloom_buffer: vec![0; width * height * 4],
            temp_buffer: vec![0; width * height * 4],
        }
    }
    
    /// Get the current configuration
    pub fn config(&self) -> &PostProcessingConfig {
        &self.config
    }
    
    /// Mutably get the configuration
    pub fn config_mut(&mut self) -> &mut PostProcessingConfig {
        &mut self.config
    }
    
    /// Trigger screen shake
    pub fn trigger_shake(&mut self, intensity: f32) {
        self.screen_shake.trigger(intensity);
    }
    
    /// Get current screen shake offset
    pub fn shake_offset(&self) -> (f32, f32) {
        (self.screen_shake.offset_x, self.screen_shake.offset_y)
    }
    
    /// Process the pixel buffer with all enabled effects
    pub fn process(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        // Update screen shake
        if self.config.screen_shake_enabled {
            self.screen_shake.update(self.config.screen_shake_decay);
        }
        
        // Apply bloom (brightness-based glow)
        if self.config.bloom_enabled {
            self.apply_bloom(pixels, width, height);
        }
        
        // Apply color grading
        if self.config.color_grading_enabled {
            self.apply_color_grading(pixels, width, height);
        }
        
        // Apply vignette
        if self.config.vignette_enabled {
            self.apply_vignette(pixels, width, height);
        }
        
        // Apply scanlines
        if self.config.scanlines_enabled {
            self.apply_scanlines(pixels, width, height);
        }
        
        // Apply chromatic aberration
        if self.config.chromatic_aberration_enabled {
            self.apply_chromatic_aberration(pixels, width, height);
        }
    }
    
    /// Apply bloom effect (glow around bright pixels)
    fn apply_bloom(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let threshold = (self.config.bloom_threshold * 255.0) as u8;
        let intensity = self.config.bloom_intensity;
        let radius = self.config.bloom_radius as usize;
        
        // Step 1: Extract bright pixels into bloom buffer
        for i in (0..pixels.len()).step_by(4) {
            let brightness = (pixels[i] as u32 + pixels[i+1] as u32 + pixels[i+2] as u32) / 3;
            if brightness as u8 > threshold {
                self.bloom_buffer[i] = pixels[i];
                self.bloom_buffer[i+1] = pixels[i+1];
                self.bloom_buffer[i+2] = pixels[i+2];
                self.bloom_buffer[i+3] = pixels[i+3];
            } else {
                self.bloom_buffer[i] = 0;
                self.bloom_buffer[i+1] = 0;
                self.bloom_buffer[i+2] = 0;
                self.bloom_buffer[i+3] = 0;
            }
        }
        
        // Step 2: Blur the bloom buffer
        for _ in 0..radius {
            self.blur_buffer(&mut self.bloom_buffer, width, height);
        }
        
        // Step 3: Add bloom back to main buffer
        for i in (0..pixels.len()).step_by(4) {
            pixels[i] = ((pixels[i] as f32 + self.bloom_buffer[i] as f32 * intensity) as u8).min(255);
            pixels[i+1] = ((pixels[i+1] as f32 + self.bloom_buffer[i+1] as f32 * intensity) as u8).min(255);
            pixels[i+2] = ((pixels[i+2] as f32 + self.bloom_buffer[i+2] as f32 * intensity) as u8).min(255);
        }
    }
    
    /// Simple box blur on the buffer
    fn blur_buffer(&mut self, buffer: &mut [u8], width: usize, height: usize) {
        let w = width;
        let h = height;
        
        // Copy to temp
        self.temp_buffer.copy_from_slice(buffer);
        
        // Box blur 3x3
        for y in 1..h-1 {
            for x in 1..w-1 {
                let idx = (y * w + x) * 4;
                
                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;
                let mut count: u32 = 0;
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as i32 + dx) as usize;
                        let ny = (y as i32 + dy) as usize;
                        if nx < w && ny < h {
                            let nidx = (ny * w + nx) * 4;
                            r += self.temp_buffer[nidx] as u32;
                            g += self.temp_buffer[nidx+1] as u32;
                            b += self.temp_buffer[nidx+2] as u32;
                            count += 1;
                        }
                    }
                }
                
                buffer[idx] = (r / count) as u8;
                buffer[idx+1] = (g / count) as u8;
                buffer[idx+2] = (b / count) as u8;
            }
        }
    }
    
    /// Apply color grading to all pixels
    fn apply_color_grading(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let mode = &self.config.color_grading_mode;
        
        for i in (0..pixels.len()).step_by(4) {
            let mut color = Color {
                r: pixels[i],
                g: pixels[i+1],
                b: pixels[i+2],
                a: pixels[i+3],
            };
            
            mode.apply(&mut color);
            
            pixels[i] = color.r;
            pixels[i+1] = color.g;
            pixels[i+2] = color.b;
        }
    }
    
    /// Apply vignette effect (darkened edges)
    fn apply_vignette(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_dist = (center_x * center_x + center_y * center_y).sqrt();
        let intensity = self.config.vignette_intensity;
        let radius = self.config.vignette_radius;
        
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt() / (max_dist * radius);
                
                // Vignette falloff
                let vignette = if dist > 1.0 {
                    1.0 - intensity
                } else {
                    1.0 - (dist * dist * intensity)
                }.max(0.0);
                
                let idx = (y * width + x) * 4;
                pixels[idx] = (pixels[idx] as f32 * vignette) as u8;
                pixels[idx+1] = (pixels[idx+1] as f32 * vignette) as u8;
                pixels[idx+2] = (pixels[idx+2] as f32 * vignette) as u8;
            }
        }
    }
    
    /// Apply scanlines (horizontal lines)
    fn apply_scanlines(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let intensity = 1.0 - self.config.scanline_intensity;
        
        for y in 0..height {
            // Every other line is dimmed
            if y % 2 == 1 {
                let row_start = y * width * 4;
                let row_end = row_start + width * 4;
                
                for i in row_start..row_end {
                    pixels[i] = (pixels[i] as f32 * intensity) as u8;
                    pixels[i+1] = (pixels[i+1] as f32 * intensity) as u8;
                    pixels[i+2] = (pixels[i+2] as f32 * intensity) as u8;
                }
            }
        }
    }
    
    /// Apply chromatic aberration (RGB channel offset)
    fn apply_chromatic_aberration(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let strength = self.config.chromatic_strength;
        let offset = (strength * 3.0) as i32;
        
        // Copy to temp
        self.temp_buffer.copy_from_slice(pixels);
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                
                // Red channel offset left
                let rx = (x as i32 - offset).max(0) as usize;
                let ry = y;
                let ridx = (ry * width + rx) * 4;
                
                // Blue channel offset right
                let bx = (x as i32 + offset).min(width as i32 - 1) as usize;
                let by = y;
                let bidx = (by * width + bx) * 4;
                
                // Apply offsets
                pixels[idx] = self.temp_buffer[ridx]; // Red from left
                pixels[idx+1] = self.temp_buffer[idx+1]; // Green center
                pixels[idx+2] = self.temp_buffer[bidx+2]; // Blue from right
            }
        }
    }
    
    /// Resize buffers (call when canvas size changes)
    pub fn resize(&mut self, width: usize, height: usize) {
        self.bloom_buffer.resize(width * height * 4, 0);
        self.temp_buffer.resize(width * height * 4, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_post_processor_creation() {
        let pp = PostProcessor::new(100, 100);
        assert_eq!(pp.config().bloom_enabled, true);
        assert_eq!(pp.config().screen_shake_enabled, true);
    }
    
    #[test]
    fn test_screen_shake() {
        let mut shake = ScreenShake::default();
        shake.trigger(0.5);
        assert!(shake.intensity > 0.0);
        
        shake.update(0.9);
        assert!(shake.intensity < 0.5);
    }
    
    #[test]
    fn test_color_grading() {
        let mut color = Color::rgb(200, 100, 50);
        
        ColorGradingMode::Vibrant.apply(&mut color);
        // Vibrant should boost saturation
        assert!(color.r >= 200);
        
        let mut color2 = Color::rgb(100, 100, 100);
        ColorGradingMode::Warm.apply(&mut color2);
        // Warm should increase red
        assert!(color2.r > color2.b);
    }
    
    #[test]
    fn test_color_grading_retro() {
        // Retro should reduce to 16-color steps
        let mut color = Color::rgb(128, 128, 128);
        ColorGradingMode::Retro.apply(&mut color);
        
        // Should be divisible by 16
        assert_eq!(color.r % 16, 0);
        assert_eq!(color.g % 16, 0);
        assert_eq!(color.b % 16, 0);
    }
}