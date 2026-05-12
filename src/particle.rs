//! Particle data structures
//! 
//! Core particle model with material, temperature, and state flags.

use std::time::{SystemTime, UNIX_EPOCH};

/// Default ambient temperature (room temperature in Kelvin)
pub const AMBIENT_TEMP: f32 = 293.15; // ~20°C

/// Particle material types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material {
    /// Empty space / air
    Air,
    /// Sand particles - falls down, piles up
    Sand,
    /// Water - flows, fills containers
    Water,
    /// Solid - immovable obstacle
    Stone,
    /// Fire - rises, spreads to flammable materials
    Fire,
    /// Smoke - rises and dissipates
    Smoke,
}

impl Material {
    /// Returns true if the material can be displaced
    pub fn is_fluid(&self) -> bool {
        matches!(self, Material::Sand | Material::Water)
    }
    
    /// Returns true if the material falls with gravity
    pub fn has_gravity(&self) -> bool {
        matches!(self, Material::Sand | Material::Water)
    }
    
    /// Returns true if the material is flammable (can be ignited)
    pub fn is_flammable(&self) -> bool {
        matches!(self, Material::Sand | Material::Water | Material::Stone | Material::Smoke)
    }
    
    /// Returns true if material rises (fire, smoke)
    pub fn rises(&self) -> bool {
        matches!(self, Material::Fire | Material::Smoke)
    }
    
    /// Default temperature for this material
    pub fn default_temp(&self) -> f32 {
        match self {
            Material::Fire => 1200.0,  // ~900°C
            Material::Smoke => 500.0, // ~230°C
            _ => AMBIENT_TEMP,
        }
    }
}

/// Particle flags for special behaviors
#[derive(Debug, Clone, Copy, Default)]
pub struct ParticleFlags {
    /// Particle is on fire (spreads to others)
    pub burning: bool,
    /// Particle has been marked for removal
    pub remove: bool,
}

/// A single particle in the simulation
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    /// Material type
    pub material: Material,
    /// Velocity (x, y) - reserved for future velocity-based physics
    pub velocity: (f32, f32),
    /// Temperature in Kelvin
    pub temperature: f32,
    /// Special state flags
    pub flags: ParticleFlags,
    /// Lifetime counter for transient particles (fire, smoke)
    pub lifetime: u32,
}

impl Particle {
    /// Create a new particle with the given material
    pub fn new(material: Material) -> Self {
        Self {
            material,
            velocity: (0.0, 0.0),
            temperature: material.default_temp(),
            flags: ParticleFlags::default(),
            lifetime: Self::default_lifetime(material),
        }
    }
    
    /// Create an empty (air) particle
    pub fn empty() -> Self {
        Self::new(Material::Air)
    }
    
    /// Default lifetime based on material type
    fn default_lifetime(material: Material) -> u32 {
        match material {
            Material::Fire => 30 + (rand_u32() % 20),  // 30-50 ticks
            Material::Smoke => 60 + (rand_u32() % 40), // 60-100 ticks
            _ => 0,
        }
    }
    
    /// Check if particle should be removed
    pub fn is_dead(&self) -> bool {
        self.material == Material::Air || self.flags.remove || self.lifetime == 0
    }
    
    /// Ignite the particle (convert to fire)
    pub fn ignite(&mut self) {
        self.material = Material::Fire;
        self.temperature = Material::Fire.default_temp();
        self.lifetime = Self::default_lifetime(Material::Fire);
        self.flags.burning = true;
    }
}

/// Simple pseudo-random number generator
fn rand_u32() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(42)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_particle_creation() {
        let sand = Particle::new(Material::Sand);
        assert_eq!(sand.material, Material::Sand);
        assert_eq!(sand.velocity, (0.0, 0.0));
        assert_eq!(sand.temperature, AMBIENT_TEMP);
    }
    
    #[test]
    fn test_fire_has_lifetime() {
        let fire = Particle::new(Material::Fire);
        assert!(fire.lifetime >= 30);
        assert!(fire.lifetime <= 50);
    }
    
    #[test]
    fn test_smoke_has_lifetime() {
        let smoke = Particle::new(Material::Smoke);
        assert!(smoke.lifetime >= 60);
        assert!(smoke.lifetime <= 100);
    }
    
    #[test]
    fn test_ignite() {
        let mut sand = Particle::new(Material::Sand);
        sand.ignite();
        assert_eq!(sand.material, Material::Fire);
        assert!(sand.flags.burning);
    }
    
    #[test]
    fn test_material_properties() {
        assert!(Material::Sand.has_gravity());
        assert!(Material::Water.has_gravity());
        assert!(!Material::Stone.has_gravity());
        
        assert!(Material::Fire.rises());
        assert!(Material::Smoke.rises());
        assert!(!Material::Sand.rises());
    }
}
