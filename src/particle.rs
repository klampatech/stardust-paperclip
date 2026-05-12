//! Particle data structures
//! 
//! Core particle model with material, temperature, and state flags.

use std::time::{SystemTime, UNIX_EPOCH};

/// Default ambient temperature (room temperature in Kelvin)
pub const AMBIENT_TEMP: f32 = 293.15; // ~20°C

/// Temperature constants (Kelvin)
pub const WATER_FREEZE_TEMP: f32 = 273.15;  // 0°C
pub const WATER_BOIL_TEMP: f32 = 373.15;    // 100°C
pub const WOOD_IGNITE_TEMP: f32 = 573.15;  // ~300°C
pub const SAND_MELT_TEMP: f32 = 2000.0;    // ~1700°C
pub const LAVA_TEMP: f32 = 1500.0;         // ~1200°C

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
    /// Black Hole - extreme gravity well
    BlackHole,
    /// Steam - rises fast, created when water meets heat
    Steam,
    /// Ice - sinks, melts when heated, slippery
    Ice,
    /// Oil - flammable liquid, flows slower than water
    Oil,
    /// Wood - solid, flammable, burns slowly
    Wood,
    /// Lava - hot molten rock, flows slowly
    Lava,
    /// Ash - residue from burned materials
    Ash,
}

/// Black hole properties
#[derive(Debug, Clone, Copy, Default)]
pub struct BlackHoleProps {
    /// Gravitational constant multiplier
    pub gravity_strength: f32,
    /// Radius of the event horizon (particles inside are consumed)
    pub event_horizon_radius: f32,
    /// Radius of gravitational influence
    pub influence_radius: f32,
    /// Hawking radiation emission rate (ticks between emissions)
    pub hawking_rate: u32,
    /// Tidal force strength (for spaghettification)
    pub tidal_strength: f32,
    /// Accretion disk formation radius
    pub accretion_radius: f32,
}

impl BlackHoleProps {
    /// Create a new black hole with default properties
    pub fn new() -> Self {
        Self {
            gravity_strength: 1000.0,
            event_horizon_radius: 3.0,
            influence_radius: 30.0,
            hawking_rate: 15,
            tidal_strength: 2.0,
            accretion_radius: 8.0,
        }
    }
}

impl Material {
    /// Returns true if the material can be displaced
    pub fn is_fluid(&self) -> bool {
        matches!(self, Material::Sand | Material::Water | Material::Oil | Material::Lava)
    }
    
    /// Returns true if the material falls with gravity
    pub fn has_gravity(&self) -> bool {
        matches!(self, Material::Sand | Material::Water | Material::Oil | Material::Ice)
    }
    
    /// Returns true if the material is flammable (can be ignited)
    pub fn is_flammable(&self) -> bool {
        matches!(self, Material::Sand | Material::Water | Material::Stone | Material::Smoke | 
                 Material::Oil | Material::Wood)
    }
    
    /// Returns true if material rises (fire, smoke, steam)
    pub fn rises(&self) -> bool {
        matches!(self, Material::Fire | Material::Smoke | Material::Steam)
    }
    
    /// Returns true if material is hot and can ignite others
    pub fn is_hot(&self) -> bool {
        matches!(self, Material::Fire | Material::Lava)
    }
    
    /// Returns true if material should be immune to fire (won't burn)
    pub fn fire_immune(&self) -> bool {
        matches!(self, Material::Stone | Material::Water | Material::Ice | 
                 Material::BlackHole | Material::Fire | Material::Steam | Material::Ash)
    }
    
    /// Default temperature for this material
    pub fn default_temp(&self) -> f32 {
        match self {
            Material::Fire => 1200.0,  // ~900°C
            Material::Smoke => 500.0, // ~230°C
            Material::BlackHole => 0.0, // Absolute zero at singularity
            Material::Lava => LAVA_TEMP, // ~1200°C
            Material::Steam => WATER_BOIL_TEMP + 50.0, // Hot
            Material::Ice => WATER_FREEZE_TEMP - 10.0, // Cold
            Material::Oil => AMBIENT_TEMP, // Room temp
            _ => AMBIENT_TEMP,
        }
    }
    
    /// Check if material has mass for gravitational calculations
    pub fn has_mass(&self) -> bool {
        !matches!(self, Material::Air | Material::BlackHole)
    }
    
    /// Get mass value for gravity calculations (higher = less affected by gravity)
    pub fn mass(&self) -> f32 {
        match self {
            Material::Sand => 2.0,
            Material::Water => 1.5,
            Material::Oil => 1.3,     // Lighter than water
            Material::Ice => 1.6,     // Denser than water (sinks)
            Material::Stone => 5.0,   // Heavy - less affected
            Material::Fire => 0.3,   // Light - more affected
            Material::Smoke => 0.1,  // Very light - most affected
            Material::Steam => 0.2,   // Very light - rises fast
            Material::Wood => 2.5,   // Solid - moderate mass
            Material::Lava => 4.0,   // Very dense - flows slowly
            Material::Ash => 1.5,    // Light - residue from burning
            _ => 0.0,
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
            Material::Fire => 30 + (rand_u32() % 20),      // 30-50 ticks
            Material::Smoke => 60 + (rand_u32() % 40),    // 60-100 ticks
            Material::Steam => 40 + (rand_u32() % 20),     // 40-60 ticks
            Material::Oil => 60 + (rand_u32() % 20),      // 60-80 ticks (burns longer)
            Material::Wood => 100 + (rand_u32() % 50),    // 100-150 ticks (slow burn)
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
