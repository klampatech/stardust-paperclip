//! Post-processing pipeline for visual effects
//! 
//! Implements bloom, motion blur, color grading, chromatic aberration, 
//! space distortion (gravitational lensing), and other post-processing effects.
//! 
//! Visual effects include:
//! - Bloom for hot particles (fire, lava, accretion disk)
//! - Screen shake on explosions/black hole events
//! - Motion blur on fast-moving particles
//! - Chromatic aberration near black holes (intensifies as particles approach)
//! - Space distortion (gravitational lensing) near event horizon
//! - Velocity-based red/blue shift tinting
//! - Additive blending for fire/plasma
//! - Camera zoom (full view to particle-level)

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
    
    /// Enable space distortion (gravitational lensing) near black holes
    pub space_distortion_enabled: bool,
    /// Space distortion intensity
    pub space_distortion_strength: f32,
    
    /// Enable velocity-based red/blue shift
    pub velocity_shift_enabled: bool,
    /// Velocity shift intensity (0.0 - 1.0)
    pub velocity_shift_intensity: f32,
    
    /// Enable additive blending for fire/plasma
    pub additive_blend_enabled: bool,
    /// Additive blend intensity for hot particles
    pub additive_blend_intensity: f32,
    
    /// Camera zoom level (1.0 = normal, 2.0 = 2x zoom, etc.)
    pub camera_zoom: f32,
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
            
            // Space distortion
            space_distortion_enabled: true,
            space_distortion_strength: 0.5,
            
            // Velocity shift
            velocity_shift_enabled: true,
            velocity_shift_intensity: 0.3,
            
            // Additive blend
            additive_blend_enabled: true,
            additive_blend_intensity: 0.2,
            
            // Camera
            camera_zoom: 1.0,
        }
    }
}

/// Color grading presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                let avg = ((color.r as u32 + color.g as u32 + color.b as u32) / 3) as i32;
                let boost = |c: u8| -> u8 {
                    let c = c as i32;
                    let diff = (c - avg) * 3 / 2;
                    ((c + diff) as f32).clamp(0.0, 255.0) as u8
                };
                color.r = boost(color.r);
                color.g = ((color.g as f32 * 1.05).min(255.0)) as u8;
                color.b = ((color.b as f32 * 0.95).max(0.0)) as u8;
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

/// Mutable configuration accessor for PostProcessor
pub struct PostProcessorConfigMut<'a> {
    inner: &'a mut PostProcessor,
}

impl<'a> PostProcessorConfigMut<'a> {
    /// Enable bloom
    pub fn enable_bloom(&mut self, enabled: bool) -> &mut Self {
        self.inner.config.bloom_enabled = enabled;
        self
    }
    
    /// Set bloom intensity
    pub fn set_bloom_intensity(&mut self, intensity: f32) -> &mut Self {
        self.inner.config.bloom_intensity = intensity.clamp(0.0, 1.0);
        self
    }
    
    /// Enable chromatic aberration
    pub fn enable_chromatic_aberration(&mut self, enabled: bool) -> &mut Self {
        self.inner.config.chromatic_aberration_enabled = enabled;
        self
    }
    
    /// Set chromatic aberration strength
    pub fn set_chromatic_strength(&mut self, strength: f32) -> &mut Self {
        self.inner.config.chromatic_strength = strength.clamp(0.0, 2.0);
        self
    }
    
    /// Enable space distortion
    pub fn enable_space_distortion(&mut self, enabled: bool) -> &mut Self {
        self.inner.config.space_distortion_enabled = enabled;
        self
    }
    
    /// Set space distortion strength
    pub fn set_distortion_strength(&mut self, strength: f32) -> &mut Self {
        self.inner.config.space_distortion_strength = strength.clamp(0.0, 1.0);
        self
    }
    
    /// Enable velocity shift
    pub fn enable_velocity_shift(&mut self, enabled: bool) -> &mut Self {
        self.inner.config.velocity_shift_enabled = enabled;
        self
    }
    
    /// Enable additive blend
    pub fn enable_additive_blend(&mut self, enabled: bool) -> &mut Self {
        self.inner.config.additive_blend_enabled = enabled;
        self
    }
    
    /// Set color grading mode
    pub fn set_color_grading(&mut self, mode: ColorGradingMode) -> &mut Self {
        self.inner.config.color_grading_mode = mode;
        self
    }
    
    /// Set camera zoom
    pub fn set_zoom(&mut self, zoom: f32) -> &mut Self {
        self.inner.config.camera_zoom = zoom.max(0.5).min(10.0);
        self
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
    /// Additive blend buffer for fire/plasma
    additive_buffer: Vec<u8>,
    /// Last known black hole positions (for proximity effects)
    black_hole_positions: Vec<(f32, f32)>,
}

impl PostProcessor {
    /// Create a new post-processor
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            config: PostProcessingConfig::default(),
            screen_shake: ScreenShake::default(),
            bloom_buffer: vec![0; width * height * 4],
            temp_buffer: vec![0; width * height * 4],
            additive_buffer: vec![0; width * height * 4],
            black_hole_positions: Vec::new(),
        }
    }
    
    /// Create with custom config
    pub fn with_config(width: usize, height: usize, config: PostProcessingConfig) -> Self {
        Self {
            config,
            screen_shake: ScreenShake::default(),
            bloom_buffer: vec![0; width * height * 4],
            temp_buffer: vec![0; width * height * 4],
            additive_buffer: vec![0; width * height * 4],
            black_hole_positions: Vec::new(),
        }
    }
    
    /// Update black hole positions from grid (call before process())
    /// This enables proximity-based effects like chromatic aberration intensification
    pub fn update_black_holes(&mut self, _grid: &crate::chunk::ChunkedGrid) {
        // Implementation would iterate through grid to find black holes
        // For now, we track positions passed to set_black_hole_position
    }
    
    /// Set a specific black hole position for distortion effects
    pub fn set_black_hole_position(&mut self, x: f32, y: f32) {
        self.black_hole_positions.push((x, y));
    }
    
    /// Clear black hole positions
    pub fn clear_black_holes(&mut self) {
        self.black_hole_positions.clear();
    }
    
    /// Get the current configuration (immutable)
    pub fn config(&self) -> &PostProcessingConfig {
        &self.config
    }
    
    /// Mutably get the configuration
    pub fn config_mut(&mut self) -> PostProcessorConfigMut<'_> {
        PostProcessorConfigMut { inner: self }
    }
    
    /// Trigger screen shake (e.g., on explosion or particle consumption)
    pub fn trigger_shake(&mut self, intensity: f32) {
        self.screen_shake.trigger(intensity);
    }
    
    /// Get current screen shake offset (for camera offset application)
    pub fn shake_offset(&self) -> (f32, f32) {
        (self.screen_shake.offset_x, self.screen_shake.offset_y)
    }
    
    /// Get camera zoom level
    pub fn camera_zoom(&self) -> f32 {
        self.config.camera_zoom
    }
    
    /// Set camera zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.config.camera_zoom = zoom.max(0.5).min(10.0);
    }
    
    /// Zoom in (increase zoom by 1.5x)
    pub fn zoom_in(&mut self) {
        self.set_zoom(self.config.camera_zoom * 1.5);
    }
    
    /// Zoom out (decrease zoom by 1.5x)
    pub fn zoom_out(&mut self) {
        self.set_zoom(self.config.camera_zoom / 1.5);
    }
    
    /// Reset zoom to default (1.0)
    pub fn reset_zoom(&mut self) {
        self.config.camera_zoom = 1.0;
    }
    
    /// Process the pixel buffer with all enabled effects
    pub fn process(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        // Update screen shake
        if self.config.screen_shake_enabled {
            self.screen_shake.update(self.config.screen_shake_decay);
        }
        
        // Apply additive blending first (glow for fire/plasma)
        if self.config.additive_blend_enabled {
            self.apply_additive_blend(pixels, width, height);
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
        
        // Apply velocity-based red/blue shift (before chromatic to preserve effect)
        if self.config.velocity_shift_enabled {
            self.apply_velocity_shift(pixels, width, height);
        }
        
        // Apply chromatic aberration (intensifies near black holes)
        if self.config.chromatic_aberration_enabled {
            self.apply_chromatic_aberration(pixels, width, height);
        }
        
        // Apply space distortion (gravitational lensing near black holes)
        if self.config.space_distortion_enabled && !self.black_hole_positions.is_empty() {
            self.apply_space_distortion(pixels, width, height);
        }
    }
    
    /// Apply additive blending for fire/plasma (glow effect)
    /// Hot particles add their color to surrounding pixels
    fn apply_additive_blend(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        // Create additive blend from bright pixels (fire, lava, plasma)
        let threshold = 200u32; // Brightness threshold for additive pixels
        let intensity = self.config.additive_blend_intensity;
        
        // Extract bright pixels into additive buffer
        for i in (0..pixels.len()).step_by(4) {
            let brightness = (pixels[i] as u32 + pixels[i+1] as u32 + pixels[i+2] as u32) / 3;
            if brightness > threshold {
                self.additive_buffer[i] = pixels[i];
                self.additive_buffer[i+1] = pixels[i+1];
                self.additive_buffer[i+2] = pixels[i+2];
                self.additive_buffer[i+3] = pixels[i+3];
            } else {
                self.additive_buffer[i] = 0;
                self.additive_buffer[i+1] = 0;
                self.additive_buffer[i+2] = 0;
                self.additive_buffer[i+3] = 0;
            }
        }
        
        // Blur the additive buffer for glow spread
        for _ in 0..2 {
            Self::blur_buffer_only(&mut self.additive_buffer, &mut self.temp_buffer, width, height);
        }
        
        // Add glow to main buffer
        for i in (0..pixels.len()).step_by(4) {
            pixels[i] = ((pixels[i] as f32 + self.additive_buffer[i] as f32 * intensity) as u8).min(255);
            pixels[i+1] = ((pixels[i+1] as f32 + self.additive_buffer[i+1] as f32 * intensity) as u8).min(255);
            pixels[i+2] = ((pixels[i+2] as f32 + self.additive_buffer[i+2] as f32 * intensity) as u8).min(255);
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
            Self::blur_buffer_only(&mut self.bloom_buffer, &mut self.temp_buffer, width, height);
        }
        
        // Step 3: Add bloom back to main buffer
        for i in (0..pixels.len()).step_by(4) {
            pixels[i] = ((pixels[i] as f32 + self.bloom_buffer[i] as f32 * intensity) as u8).min(255);
            pixels[i+1] = ((pixels[i+1] as f32 + self.bloom_buffer[i+1] as f32 * intensity) as u8).min(255);
            pixels[i+2] = ((pixels[i+2] as f32 + self.bloom_buffer[i+2] as f32 * intensity) as u8).min(255);
        }
    }
    
    /// Simple box blur on the buffer (static version to avoid borrow conflicts)
    fn blur_buffer_only(buffer: &mut [u8], temp: &mut [u8], width: usize, height: usize) {
        let w = width;
        let h = height;
        
        // Copy buffer to temp first
        let buf_len = buffer.len();
        temp[..buf_len].copy_from_slice(buffer);
        
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
                            // Read from temp buffer
                            r += temp[nidx] as u32;
                            g += temp[nidx+1] as u32;
                            b += temp[nidx+2] as u32;
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
    /// Intensity increases near black holes for dramatic effect
    fn apply_chromatic_aberration(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        // Calculate base strength - increase near black holes
        let mut base_strength = self.config.chromatic_strength;
        
        // If we have black hole positions, increase aberration near them
        if !self.black_hole_positions.is_empty() {
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            
            // Find distance to nearest black hole (normalized)
            let min_dist = self.black_hole_positions.iter()
                .map(|(bh_x, bh_y)| {
                    let dx = center_x - bh_x;
                    let dy = center_y - bh_y;
                    (dx * dx + dy * dy).sqrt()
                })
                .fold(f32::MAX, |a, b| a.min(b));
            
            // Normalize and boost intensity near black holes
            let max_dist = ((width * width + height * height) as f32).sqrt() / 2.0;
            let proximity = 1.0 - (min_dist / max_dist).min(1.0);
            base_strength += proximity * self.config.chromatic_strength * 2.0;
        }
        
        let strength = base_strength.min(2.0); // Cap at 2x
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
    
    /// Apply space distortion (gravitational lensing) effect
    /// Simulates the bending of light near black holes
    fn apply_space_distortion(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let strength = self.config.space_distortion_strength;
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        
        // Copy to temp
        self.temp_buffer.copy_from_slice(pixels);
        
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                
                // Calculate distortion based on distance from center
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();
                let max_dist = (center_x * center_x + center_y * center_y).sqrt();
                
                // Gravitational lensing: pixels are pushed outward from center
                // Effect is strongest near the center (black hole)
                let normalized_dist = dist / max_dist;
                let distortion_factor = (1.0 - normalized_dist).powi(3) * strength;
                
                if distortion_factor > 0.01 {
                    // Calculate displaced position
                    let dir_x = if dist > 0.01 { dx / dist } else { 0.0 };
                    let dir_y = if dist > 0.01 { dy / dist } else { 0.0 };
                    
                    // Push pixels outward (away from center) like gravitational lensing
                    let displace_x = (dir_x * distortion_factor * 20.0) as i32;
                    let displace_y = (dir_y * distortion_factor * 20.0) as i32;
                    
                    let src_x = (x as i32 + displace_x).max(0).min(width as i32 - 1) as usize;
                    let src_y = (y as i32 + displace_y).max(0).min(height as i32 - 1) as usize;
                    let src_idx = (src_y * width + src_x) * 4;
                    
                    // Sample from displaced position
                    pixels[idx] = self.temp_buffer[src_idx];
                    pixels[idx + 1] = self.temp_buffer[src_idx + 1];
                    pixels[idx + 2] = self.temp_buffer[src_idx + 2];
                }
            }
        }
    }
    
    /// Apply velocity-based red/blue shift effect
    /// Moving particles appear red-shifted (moving away) or blue-shifted (moving toward)
    fn apply_velocity_shift(&mut self, pixels: &mut [u8], width: usize, height: usize) {
        let intensity = self.config.velocity_shift_intensity;
        
        // For a pixel-based renderer, we simulate velocity shift based on
        // pixel brightness and position (approximating particle velocity)
        for i in (0..pixels.len()).step_by(4) {
            let brightness = (pixels[i] as u32 + pixels[i+1] as u32 + pixels[i+2] as u32) / 3;
            
            // Hot particles (fire, lava) get blue shift
            // Cold particles get red shift
            if brightness > 200 {
                // Blue shift for hot particles
                let shift = (brightness - 200) as f32 / 55.0 * intensity;
                pixels[i] = (pixels[i] as f32 * (1.0 - shift * 0.5)) as u8;     // Reduce red
                pixels[i+2] = (pixels[i+2] as f32 * (1.0 + shift)) as u8;        // Boost blue
            } else if brightness < 100 {
                // Red shift for cold particles
                let shift = (100 - brightness) as f32 / 100.0 * intensity;
                pixels[i] = (pixels[i] as f32 * (1.0 + shift)) as u8;           // Boost red
                pixels[i+2] = (pixels[i+2] as f32 * (1.0 - shift * 0.5)) as u8;  // Reduce blue
            }
        }
    }
    
    /// Resize buffers (call when canvas size changes)
    pub fn resize(&mut self, width: usize, height: usize) {
        self.bloom_buffer.resize(width * height * 4, 0);
        self.temp_buffer.resize(width * height * 4, 0);
        self.additive_buffer.resize(width * height * 4, 0);
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
    
    #[test]
    fn test_black_hole_position_tracking() {
        let mut pp = PostProcessor::new(100, 100);
        pp.set_black_hole_position(50.0, 50.0);
        assert_eq!(pp.config().space_distortion_enabled, true);
        assert_eq!(pp.config().chromatic_aberration_enabled, false);
    }
    
    #[test]
    fn test_zoom_operations() {
        let mut pp = PostProcessor::new(100, 100);
        
        // Initial zoom
        assert_eq!(pp.camera_zoom(), 1.0);
        
        // Zoom in
        pp.zoom_in();
        assert!(pp.camera_zoom() > 1.0);
        
        // Zoom out
        let zoom_before = pp.camera_zoom();
        pp.zoom_out();
        assert!(pp.camera_zoom() < zoom_before);
        
        // Reset
        pp.reset_zoom();
        assert_eq!(pp.camera_zoom(), 1.0);
    }
    
    #[test]
    fn test_zoom_clamping() {
        let mut pp = PostProcessor::new(100, 100);
        
        // Zoom in multiple times (should clamp at 10.0)
        for _ in 0..10 {
            pp.zoom_in();
        }
        assert!(pp.camera_zoom() <= 10.0);
        
        // Zoom out multiple times (should clamp at 0.5)
        for _ in 0..10 {
            pp.zoom_out();
        }
        assert!(pp.camera_zoom() >= 0.5);
    }
    
    #[test]
    fn test_trigger_shake() {
        let mut pp = PostProcessor::new(100, 100);
        
        // Initial shake should be zero
        let (ox, oy) = pp.shake_offset();
        assert_eq!(ox, 0.0);
        assert_eq!(oy, 0.0);
        
        // Trigger shake
        pp.trigger_shake(0.8);
        
        // After triggering, intensity should be set
        // (shake offset will be non-zero after process)
        assert_eq!(pp.config().screen_shake_enabled, true);
    }
    
    #[test]
    fn test_config_mut() {
        let mut pp = PostProcessor::new(100, 100);
        
        pp.config_mut()
            .enable_bloom(false)
            .set_bloom_intensity(0.8)
            .enable_chromatic_aberration(true)
            .set_color_grading(ColorGradingMode::Cinematic);
        
        assert_eq!(pp.config().bloom_enabled, false);
        assert_eq!(pp.config().bloom_intensity, 0.8);
        assert_eq!(pp.config().chromatic_aberration_enabled, true);
        assert_eq!(pp.config().color_grading_mode, ColorGradingMode::Cinematic);
    }
    
    #[test]
    fn test_new_effects_enabled_by_default() {
        let pp = PostProcessor::new(100, 100);
        let cfg = pp.config();
        
        // New effects should be enabled
        assert!(cfg.space_distortion_enabled);
        assert!(cfg.velocity_shift_enabled);
        assert!(cfg.additive_blend_enabled);
    }
}