//! Space game object models
//! 
//! Defines high-level game objects that exist in the space simulation.
//! These composite objects contain particles and have physics properties.

use super::particle::{Material, Particle, BlackHoleProps};
use super::grid::Grid;
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for game objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameObjectId(pub u64);

impl GameObjectId {
    pub fn new() -> Self {
        Self(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(42))
    }
}

/// Types of space game objects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameObjectType {
    /// A celestial body (planet, moon, asteroid)
    Celestial,
    /// A star (provides light, heat, gravity)
    Star,
    /// A spaceship or spacecraft
    Spacecraft,
    /// A space station or habitat
    Station,
    /// A comet or ice body
    Comet,
    /// An artificial structure (debris, satellite)
    Debris,
    /// Nebula cloud (particle cluster)
    Nebula,
}

impl GameObjectType {
    pub fn name(&self) -> &'static str {
        match self {
            GameObjectType::Celestial => "Celestial Body",
            GameObjectType::Star => "Star",
            GameObjectType::Spacecraft => "Spacecraft",
            GameObjectType::Station => "Space Station",
            GameObjectType::Comet => "Comet",
            GameObjectType::Debris => "Debris",
            GameObjectType::Nebula => "Nebula",
        }
    }
    
    pub fn is_natural(&self) -> bool {
        matches!(self, 
            GameObjectType::Celestial | 
            GameObjectType::Star | 
            GameObjectType::Comet |
            GameObjectType::Nebula
        )
    }
}

/// Position in 2D space (grid coordinates)
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn direction_to(&self, other: &Position) -> (f32, f32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            (dx / dist, dy / dist)
        } else {
            (0.0, 0.0)
        }
    }
}

/// Velocity vector in 2D space
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }
    
    pub fn speed(&self) -> f32 {
        (self.dx * self.dx + self.dy * self.dy).sqrt()
    }
    
    pub fn add(&mut self, other: &Velocity) {
        self.dx += other.dx;
        self.dy += other.dy;
    }
    
    pub fn scale(&mut self, factor: f32) {
        self.dx *= factor;
        self.dy *= factor;
    }
}

/// Base properties for all space objects
#[derive(Debug, Clone)]
pub struct SpaceObjectProps {
    /// Object mass for gravity calculations
    pub mass: f32,
    /// Radius of the object (bounding)
    pub radius: f32,
    /// Temperature in Kelvin
    pub temperature: f32,
    /// Color for rendering (RGBA)
    pub color: (u8, u8, u8, u8),
    /// Glow intensity (0-1)
    pub glow: f32,
}

impl Default for SpaceObjectProps {
    fn default() -> Self {
        Self {
            mass: 1.0,
            radius: 1.0,
            temperature: 293.15, // Room temp
            color: (128, 128, 128, 255),
            glow: 0.0,
        }
    }
}

impl SpaceObjectProps {
    pub fn new(mass: f32, radius: f32) -> Self {
        Self {
            mass,
            radius,
            ..Default::default()
        }
    }
    
    /// Set color
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b, 255);
        self
    }
    
    /// Set temperature
    pub fn with_temp(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }
    
    /// Set glow intensity
    pub fn with_glow(mut self, glow: f32) -> Self {
        self.glow = glow.clamp(0.0, 1.0);
        self
    }
}

/// Celestial body properties (planets, moons, asteroids)
#[derive(Debug, Clone)]
pub struct CelestialProps {
    /// Type of celestial body
    pub body_type: CelestialType,
    /// Surface material type
    pub surface: Material,
    /// Has atmosphere
    pub has_atmosphere: bool,
    /// Atmosphere thickness (0-1)
    pub atmosphere_thickness: f32,
    /// Has magnetic field
    pub has_magnetic_field: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelestialType {
    Planet,
    Moon,
    Asteroid,
    DwarfPlanet,
}

impl Default for CelestialProps {
    fn default() -> Self {
        Self {
            body_type: CelestialType::Planet,
            surface: Material::Stone,
            has_atmosphere: false,
            atmosphere_thickness: 0.0,
            has_magnetic_field: false,
        }
    }
}

impl CelestialProps {
    pub fn planet() -> Self {
        Self {
            body_type: CelestialType::Planet,
            surface: Material::Stone,
            has_atmosphere: true,
            atmosphere_thickness: 0.5,
            has_magnetic_field: true,
        }
    }
    
    pub fn moon() -> Self {
        Self {
            body_type: CelestialType::Moon,
            surface: Material::Stone,
            has_atmosphere: false,
            atmosphere_thickness: 0.0,
            has_magnetic_field: false,
        }
    }
    
    pub fn asteroid(material: Material) -> Self {
        Self {
            body_type: CelestialType::Asteroid,
            surface: material,
            has_atmosphere: false,
            atmosphere_thickness: 0.0,
            has_magnetic_field: false,
        }
    }
}

/// Star properties (provides light, heat, gravity)
#[derive(Debug, Clone)]
pub struct StarProps {
    /// Stellar classification
    pub class: StarClass,
    /// Luminosity (relative to sun)
    pub luminosity: f32,
    /// Solar mass (relative to sun)
    pub solar_mass: f32,
    /// Core temperature in Kelvin
    pub core_temp: f32,
    /// Surface temperature in Kelvin
    pub surface_temp: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarClass {
    O, // Blue supergiant (>30k K)
    B, // Blue (10-30k K)
    A, // White (7.5-10k K)
    F, // Yellow-white (6-7.5k K)
    G, // Yellow (5.2-6k K) - like our sun
    K, // Orange (3.7-5.2k K)
    M, // Red dwarf (<3.7k K)
}

impl StarClass {
    pub fn temperature(&self) -> f32 {
        match self {
            StarClass::O => 40000.0,
            StarClass::B => 20000.0,
            StarClass::A => 8500.0,
            StarClass::F => 6750.0,
            StarClass::G => 5778.0, // Sun
            StarClass::K => 4500.0,
            StarClass::M => 3000.0,
        }
    }
    
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            StarClass::O => (155, 175, 255), // Blue
            StarClass::B => (170, 185, 255), // Blue-white
            StarClass::A => (235, 240, 255), // White
            StarClass::F => (250, 250, 255), // Yellow-white
            StarClass::G => (255, 250, 230), // Yellow (like sun)
            StarClass::K => (255, 210, 170), // Orange
            StarClass::M => (255, 180, 120),  // Red
        }
    }
}

impl Default for StarProps {
    fn default() -> Self {
        Self {
            class: StarClass::G,
            luminosity: 1.0,
            solar_mass: 1.0,
            core_temp: 15_000_000.0, // Sun core
            surface_temp: 5778.0,
        }
    }
}

impl StarProps {
    pub fn sun() -> Self {
        Self {
            class: StarClass::G,
            luminosity: 1.0,
            solar_mass: 1.0,
            core_temp: 15_000_000.0,
            surface_temp: 5778.0,
        }
    }
    
    pub fn red_dwarf() -> Self {
        Self {
            class: StarClass::M,
            luminosity: 0.01,
            solar_mass: 0.2,
            core_temp: 5_000_000.0,
            surface_temp: 3000.0,
        }
    }
    
    pub fn blue_giant() -> Self {
        Self {
            class: StarClass::O,
            luminosity: 100.0,
            solar_mass: 20.0,
            core_temp: 40_000_000.0,
            surface_temp: 40000.0,
        }
    }
}

/// Spacecraft properties
#[derive(Debug, Clone)]
pub struct SpacecraftProps {
    /// Ship class/type
    pub ship_class: ShipClass,
    /// Hull integrity (0-100)
    pub hull: f32,
    /// Engine power output
    pub engine_power: f32,
    /// Fuel remaining (0-100)
    pub fuel: f32,
    /// Shield strength (0-100)
    pub shields: f32,
    /// Cargo capacity
    pub cargo_capacity: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipClass {
    Scout,      // Fast, light
    Fighter,    // Combat
    Freighter,  // Cargo
    Cruiser,    // Heavy combat
    ColonyShip, // Large, slow
    Station,    // Orbital
}

impl ShipClass {
    pub fn name(&self) -> &'static str {
        match self {
            ShipClass::Scout => "Scout",
            ShipClass::Fighter => "Fighter",
            ShipClass::Freighter => "Freighter",
            ShipClass::Cruiser => "Cruiser",
            ShipClass::ColonyShip => "Colony Ship",
            ShipClass::Station => "Space Station",
        }
    }
}

impl Default for SpacecraftProps {
    fn default() -> Self {
        Self {
            ship_class: ShipClass::Scout,
            hull: 100.0,
            engine_power: 1.0,
            fuel: 100.0,
            shields: 50.0,
            cargo_capacity: 10,
        }
    }
}

impl SpacecraftProps {
    pub fn scout() -> Self {
        Self {
            ship_class: ShipClass::Scout,
            hull: 50.0,
            engine_power: 2.0,
            fuel: 100.0,
            shields: 20.0,
            cargo_capacity: 5,
        }
    }
    
    pub fn freighter() -> Self {
        Self {
            ship_class: ShipClass::Freighter,
            hull: 150.0,
            engine_power: 0.5,
            fuel: 100.0,
            shields: 30.0,
            cargo_capacity: 100,
        }
    }
}

/// Comet properties (ice body with tail)
#[derive(Debug, Clone)]
pub struct CometProps {
    /// Ice composition purity (0-1)
    pub ice_purity: f32,
    /// Tail length (particles)
    pub tail_length: u32,
    /// Tail width (particles)
    pub tail_width: f32,
    /// Volatility (explosion chance on impact)
    pub volatility: f32,
}

impl Default for CometProps {
    fn default() -> Self {
        Self {
            ice_purity: 0.8,
            tail_length: 50,
            tail_width: 5.0,
            volatility: 0.3,
        }
    }
}

/// Nebula properties (particle cloud)
#[derive(Debug, Clone)]
pub struct NebulaProps {
    /// Nebula type
    pub nebula_type: NebulaType,
    /// Particle density (0-1)
    pub density: f32,
    /// Color tint
    pub color_tint: (u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NebulaType {
    Emission,   // HII regions (pink/red)
    Reflection, // Dust clouds (blue)
    Dark,       // Absorption (black)
    Planetary,  // Around stars (varied)
}

impl Default for NebulaProps {
    fn default() -> Self {
        Self {
            nebula_type: NebulaType::Emission,
            density: 0.3,
            color_tint: (255, 100, 150),
        }
    }
}

/// Main game object structure
#[derive(Debug, Clone)]
pub struct GameObject {
    /// Unique identifier
    pub id: GameObjectId,
    /// Object type
    pub object_type: GameObjectType,
    /// Name (optional)
    pub name: Option<String>,
    /// Position
    pub position: Position,
    /// Velocity
    pub velocity: Velocity,
    /// Base properties
    pub props: SpaceObjectProps,
    /// Type-specific properties
    pub specific_props: SpecificProps,
    /// Created timestamp
    pub created_at: u64,
    /// Active/inactive
    pub active: bool,
}

#[derive(Debug, Clone)]
pub enum SpecificProps {
    /// Celestial body properties
    Celestial(CelestialProps),
    /// Star properties
    Star(StarProps),
    /// Spacecraft properties
    Spacecraft(SpacecraftProps),
    /// Comet properties
    Comet(CometProps),
    /// Nebula properties
    Nebula(NebulaProps),
    /// Generic/empty
    None,
}

impl GameObject {
    /// Create a new game object
    pub fn new(
        object_type: GameObjectType,
        position: Position,
        props: SpaceObjectProps,
        specific_props: SpecificProps,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        Self {
            id: GameObjectId::new(),
            object_type,
            name: None,
            position,
            velocity: Velocity::default(),
            props,
            specific_props,
            created_at: now,
            active: true,
        }
    }
    
    /// Create a celestial body
    pub fn celestial(
        position: Position,
        radius: f32,
        body_type: CelestialType,
        surface: Material,
    ) -> Self {
        let mass = radius * radius * 10.0; // Simplified mass calc
        let props = SpaceObjectProps::new(mass, radius)
            .with_glow(0.2);
        
        let celestial = match body_type {
            CelestialType::Planet => CelestialProps::planet(),
            CelestialType::Moon => CelestialProps::moon(),
            CelestialType::Asteroid => CelestialProps::asteroid(surface),
            CelestialType::DwarfPlanet => CelestialProps::default(),
        };
        
        Self::new(
            GameObjectType::Celestial,
            position,
            props,
            SpecificProps::Celestial(celestial),
        )
    }
    
    /// Create a star
    pub fn star(position: Position, radius: f32, class: StarClass) -> Self {
        let mass = radius * radius * 100.0;
        let (r, g, b) = class.color();
        let props = SpaceObjectProps::new(mass, radius)
            .with_color(r, g, b)
            .with_temp(class.temperature())
            .with_glow(1.0);
        
        let star = StarProps {
            class,
            luminosity: (radius / 10.0).max(0.1),
            solar_mass: radius / 10.0,
            core_temp: class.temperature() * 1000.0,
            surface_temp: class.temperature(),
        };
        
        Self::new(
            GameObjectType::Star,
            position,
            props,
            SpecificProps::Star(star),
        )
    }
    
    /// Create a spacecraft
    pub fn spacecraft(position: Position, ship_class: ShipClass) -> Self {
        let (mass, radius) = match ship_class {
            ShipClass::Scout => (10.0, 3.0),
            ShipClass::Fighter => (15.0, 4.0),
            ShipClass::Freighter => (50.0, 8.0),
            ShipClass::Cruiser => (100.0, 12.0),
            ShipClass::ColonyShip => (200.0, 15.0),
            ShipClass::Station => (300.0, 20.0),
        };
        
        let props = SpaceObjectProps::new(mass, radius)
            .with_color(200, 200, 220)
            .with_glow(0.3);
        
        let ship = match ship_class {
            ShipClass::Scout => SpacecraftProps::scout(),
            ShipClass::Freighter => SpacecraftProps::freighter(),
            _ => SpacecraftProps::default(),
        };
        
        Self::new(
            GameObjectType::Spacecraft,
            position,
            props,
            SpecificProps::Spacecraft(ship),
        )
    }
    
    /// Create a comet
    pub fn comet(position: Position, velocity: Velocity) -> Self {
        let props = SpaceObjectProps::new(5.0, 5.0)
            .with_color(200, 230, 255)
            .with_glow(0.5);
        
        Self::new(
            GameObjectType::Comet,
            position,
            props,
            SpecificProps::Comet(CometProps::default()),
        )
    }
    
    /// Create a nebula
    pub fn nebula(position: Position, radius: f32) -> Self {
        let props = SpaceObjectProps::new(radius * 5.0, radius)
            .with_glow(0.8);
        
        Self::new(
            GameObjectType::Nebula,
            position,
            props,
            SpecificProps::Nebula(NebulaProps::default()),
        )
    }
    
    /// Apply gravitational force from another object
    pub fn apply_gravity(&mut self, other: &GameObject, gravitational_constant: f32) {
        let dx = other.position.x - self.position.x;
        let dy = other.position.y - self.position.y;
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt().max(1.0);
        
        // F = G * m1 * m2 / r^2
        let force = gravitational_constant * self.props.mass * other.props.mass / dist_sq;
        
        // Acceleration = F / m
        let ax = (force * dx / dist) / self.props.mass;
        let ay = (force * dy / dist) / self.props.mass;
        
        self.velocity.dx += ax;
        self.velocity.dy += ay;
    }
    
    /// Update position based on velocity
    pub fn update_position(&mut self, dt: f32) {
        self.position.x += self.velocity.dx * dt;
        self.position.y += self.velocity.dy * dt;
    }
    
    /// Check if this object collides with another
    pub fn collides_with(&self, other: &GameObject) -> bool {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        let dist = (dx * dx + dy * dy).sqrt();
        dist < (self.props.radius + other.props.radius)
    }
    
    /// Apply damage to spacecraft
    pub fn damage(&mut self, amount: f32) -> bool {
        if let SpecificProps::Spacecraft(ref mut ship) = self.specific_props {
            ship.hull = (ship.hull - amount).max(0.0);
            ship.shields = (ship.shields - amount * 0.5).max(0.0);
            return ship.hull <= 0.0;
        }
        false
    }
    
    /// Set name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

/// Manager for all game objects
#[derive(Debug, Clone)]
pub struct GameObjectManager {
    /// All game objects
    objects: Vec<GameObject>,
    /// Object lookup by ID
    id_map: std::collections::HashMap<GameObjectId, usize>,
}

impl GameObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            id_map: std::collections::HashMap::new(),
        }
    }
    
    /// Add a game object
    pub fn add(&mut self, mut obj: GameObject) -> GameObjectId {
        let id = obj.id;
        let idx = self.objects.len();
        self.id_map.insert(id, idx);
        self.objects.push(obj);
        id
    }
    
    /// Remove a game object by ID
    pub fn remove(&mut self, id: GameObjectId) -> Option<GameObject> {
        if let Some(&idx) = self.id_map.get(&id) {
            self.id_map.remove(&id);
            // Swap-remove for efficiency
            if idx < self.objects.len() - 1 {
                let last = self.objects.pop().unwrap();
                self.objects[idx] = last;
                // Update ID map for swapped object
                if let Some(new_idx) = self.id_map.get_mut(&last.id) {
                    *new_idx = idx;
                }
            } else {
                self.objects.pop();
            }
            return Some(self.objects.swap_remove(idx));
        }
        None
    }
    
    /// Get object by ID
    pub fn get(&self, id: GameObjectId) -> Option<&GameObject> {
        self.id_map.get(&id).and_then(|&idx| self.objects.get(idx))
    }
    
    /// Get mutable object by ID
    pub fn get_mut(&mut self, id: GameObjectId) -> Option<&mut GameObject> {
        self.id_map.get(&id).and_then(move |&idx| self.objects.get_mut(idx))
    }
    
    /// Get all objects of a specific type
    pub fn get_by_type(&self, object_type: GameObjectType) -> Vec<&GameObject> {
        self.objects.iter().filter(|o| o.object_type == object_type).collect()
    }
    
    /// Get all active objects
    pub fn active_objects(&self) -> Vec<&GameObject> {
        self.objects.iter().filter(|o| o.active).collect()
    }
    
    /// Update all objects (physics)
    pub fn update(&mut self, dt: f32, gravitational_constant: f32) {
        // Update positions based on velocities
        for obj in &mut self.objects {
            obj.update_position(dt);
        }
        
        // Apply gravitational interactions between all pairs
        for i in 0..self.objects.len() {
            for j in (i + 1)..self.objects.len() {
                let mut obj_i = self.objects[i].clone();
                let mut obj_j = self.objects[j].clone();
                
                obj_i.apply_gravity(&obj_j, gravitational_constant);
                obj_j.apply_gravity(&obj_i, gravitational_constant);
                
                // Update velocities in the main vector
                if let Some(o) = self.objects.get_mut(i) {
                    o.velocity = obj_i.velocity;
                }
                if let Some(o) = self.objects.get_mut(j) {
                    o.velocity = obj_j.velocity;
                }
            }
        }
    }
    
    /// Get all objects near a position
    pub fn objects_near(&self, position: &Position, radius: f32) -> Vec<&GameObject> {
        self.objects.iter()
            .filter(|o| o.position.distance_to(position) <= radius)
            .collect()
    }
    
    /// Number of objects
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Default for GameObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_planet() {
        let planet = GameObject::celestial(
            Position::new(100.0, 100.0),
            10.0,
            CelestialType::Planet,
            Material::Stone,
        );
        
        assert_eq!(planet.object_type, GameObjectType::Celestial);
        assert!(planet.props.mass > 0.0);
        assert_eq!(planet.props.radius, 10.0);
    }
    
    #[test]
    fn test_create_star() {
        let star = GameObject::star(
            Position::new(50.0, 50.0),
            15.0,
            StarClass::G,
        );
        
        assert_eq!(star.object_type, GameObjectType::Star);
        assert!(star.props.glow > 0.5);
    }
    
    #[test]
    fn test_create_spacecraft() {
        let ship = GameObject::spacecraft(
            Position::new(200.0, 200.0),
            ShipClass::Fighter,
        );
        
        assert_eq!(ship.object_type, GameObjectType::Spacecraft);
        if let SpecificProps::Spacecraft(props) = ship.specific_props {
            assert_eq!(props.ship_class, ShipClass::Fighter);
        } else {
            panic!("Expected Spacecraft props");
        }
    }
    
    #[test]
    fn test_gravity() {
        let mut star = GameObject::star(Position::new(0.0, 0.0), 10.0, StarClass::G);
        let planet = GameObject::celestial(Position::new(100.0, 0.0), 5.0, CelestialType::Planet, Material::Stone);
        
        star.apply_gravity(&planet, 1.0);
        
        // Planet should be pulled toward star
        assert!(star.velocity.dx > 0.0 || star.velocity.dy != 0.0);
    }
    
    #[test]
    fn test_manager() {
        let mut manager = GameObjectManager::new();
        
        let star_id = manager.add(GameObject::star(Position::new(0.0, 0.0), 10.0, StarClass::G));
        let planet_id = manager.add(GameObject::celestial(Position::new(100.0, 0.0), 5.0, CelestialType::Planet, Material::Stone));
        
        assert_eq!(manager.len(), 2);
        assert!(manager.get(star_id).is_some());
        assert!(manager.get(planet_id).is_some());
        
        // Update physics
        manager.update(1.0, 1.0);
        
        // Remove planet
        let removed = manager.remove(planet_id);
        assert!(removed.is_some());
        assert_eq!(manager.len(), 1);
    }
    
    #[test]
    fn test_collision() {
        let obj1 = GameObject::celestial(Position::new(0.0, 0.0), 10.0, CelestialType::Planet, Material::Stone);
        let obj2 = GameObject::celestial(Position::new(15.0, 0.0), 10.0, CelestialType::Moon, Material::Stone);
        let obj3 = GameObject::celestial(Position::new(100.0, 0.0), 5.0, CelestialType::Asteroid, Material::Sand);
        
        assert!(obj1.collides_with(&obj2));
        assert!(!obj1.collides_with(&obj3));
    }
}