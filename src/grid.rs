//! Grid data structures
//! 
//! Core grid for particle storage with efficient get/set operations.

use crate::particle::{Material, Particle};

/// Grid dimensions
#[derive(Debug, Clone, Copy)]
pub struct GridSize {
    /// Width in cells
    pub width: usize,
    /// Height in cells
    pub height: usize,
}

impl GridSize {
    /// Create new grid dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    
    /// Check if coordinates are within bounds
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
    
    /// Calculate flat index for coordinates
    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        if self.contains(x, y) {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

/// Core simulation grid
#[derive(Debug, Clone)]
pub struct Grid {
    /// Grid dimensions
    size: GridSize,
    /// Flat particle storage
    cells: Vec<Particle>,
}

impl Grid {
    /// Create a new empty grid
    pub fn new(size: GridSize) -> Self {
        let cell_count = size.width * size.height;
        Self {
            size,
            cells: vec![Particle::empty(); cell_count],
        }
    }
    
    /// Get grid dimensions
    pub fn size(&self) -> GridSize {
        self.size
    }
    
    /// Flatten 2D coordinates to 1D index
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        self.size.index(x, y)
    }
    
    /// Get particle at position
    pub fn get(&self, x: usize, y: usize) -> Option<Particle> {
        self.index(x, y).map(|i| self.cells[i])
    }
    
    /// Set particle at position
    pub fn set(&mut self, x: usize, y: usize, particle: Particle) -> bool {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = particle;
            true
        } else {
            false
        }
    }
    
    /// Check if cell is empty (contains air)
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get(x, y)
            .map(|p| p.material == Material::Air)
            .unwrap_or(false)
    }
    
    /// Check if position is within bounds
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        self.size.contains(x, y)
    }
    
    /// Spawn a particle at position if valid
    pub fn spawn(&mut self, x: usize, y: usize, material: Material) -> bool {
        if self.is_empty(x, y) {
            self.set(x, y, Particle::new(material))
        } else {
            false
        }
    }
    
    /// Remove particle at position
    pub fn remove(&mut self, x: usize, y: usize) -> bool {
        if self.in_bounds(x, y) && !self.is_empty(x, y) {
            self.set(x, y, Particle::empty())
        } else {
            false
        }
    }
    
    /// Swap two positions
    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        if let (Some(p1), Some(p2)) = (self.get(x1, y1), self.get(x2, y2)) {
            let _ = self.set(x2, y2, p1);
            let _ = self.set(x1, y1, p2);
        }
    }
    
    /// Get total particle count
    pub fn particle_count(&self) -> usize {
        self.cells.iter().filter(|p| p.material != Material::Air).count()
    }
    
    /// Clear all particles
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Particle::empty();
        }
    }
}

/// Directional helpers for neighbor checks
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    /// Get x delta for direction
    pub fn dx(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Down => 0,
        }
    }
    
    /// Get y delta for direction
    pub fn dy(&self) -> i32 {
        match self {
            Direction::Left | Direction::Right => 0,
            Direction::Down => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(GridSize::new(10, 10));
        assert_eq!(grid.size().width, 10);
        assert_eq!(grid.size().height, 10);
        assert_eq!(grid.particle_count(), 0);
    }
    
    #[test]
    fn test_particle_operations() {
        let mut grid = Grid::new(GridSize::new(5, 5));
        
        // Spawn a sand particle
        assert!(grid.spawn(2, 2, Material::Sand));
        
        // Check it's there
        let particle = grid.get(2, 2).unwrap();
        assert_eq!(particle.material, Material::Sand);
        assert_eq!(grid.particle_count(), 1);
        
        // Try to spawn on occupied cell
        assert!(!grid.spawn(2, 2, Material::Water));
        assert_eq!(grid.particle_count(), 1);
    }
    
    #[test]
    fn test_swap() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        
        grid.spawn(0, 0, Material::Sand);
        grid.spawn(1, 1, Material::Water);
        
        grid.swap(0, 0, 1, 1);
        
        assert_eq!(grid.get(0, 0).unwrap().material, Material::Water);
        assert_eq!(grid.get(1, 1).unwrap().material, Material::Sand);
    }
    
    #[test]
    fn test_bounds() {
        let grid = Grid::new(GridSize::new(5, 5));
        
        assert!(grid.in_bounds(0, 0));
        assert!(grid.in_bounds(4, 4));
        assert!(!grid.in_bounds(5, 0));
        assert!(!grid.in_bounds(0, 5));
    }
}
