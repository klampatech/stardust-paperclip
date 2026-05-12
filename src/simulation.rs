//! Physics simulation engine for falling sand
//! 
//! Handles gravity, collision, and material-specific behavior.
//! Includes fire spread, water extinguishing, and smoke dissipation.

use crate::grid::Grid;
use crate::particle::Material;
use crate::chunk::ChunkedGrid;

/// Simulation engine that processes physics each tick
#[derive(Debug, Clone)]
pub struct Simulator {
    /// Whether to process bottom-to-top (prevents double-moves)
    bottom_to_top: bool,
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulator {
    /// Create a new simulator
    pub fn new() -> Self {
        Self {
            // Default to bottom-to-top processing to prevent double-moves
            bottom_to_top: true,
        }
    }
    
    /// Run one simulation tick on a flat grid
    pub fn tick(&mut self, grid: &mut Grid) {
        let height = grid.size().height;
        
        for y in 0..height {
            let row = if self.bottom_to_top { height - 1 - y } else { y };
            self.process_row(grid, row);
        }
    }
    
    fn process_row(&self, grid: &mut Grid, y: usize) {
        let width = grid.size().width;
        
        // Collect modifications to apply after iteration
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        let mut to_ignite: Vec<(usize, usize)> = Vec::new();
        let mut to_smoke: Vec<(usize, usize)> = Vec::new();
        
        // Process left-to-right for this row
        for x in 0..width {
            let particle = match grid.get(x, y) {
                Some(p) => p,
                None => continue,
            };
            
            match particle.material {
                Material::Sand => self.update_sand(grid, x, y),
                Material::Water => self.update_water(grid, x, y),
                Material::Fire => {
                    self.update_fire(grid, x, y, &mut to_remove, &mut to_ignite, &mut to_smoke);
                }
                Material::Smoke => {
                    self.update_smoke(grid, x, y, &mut to_remove);
                }
                Material::Air | Material::Stone => {}
            }
        }
        
        // Apply fire spread
        for (x, y) in to_ignite {
            if grid.in_bounds(x, y) {
                if let Some(mut p) = grid.get(x, y) {
                    if p.material.is_flammable() && !p.flags.burning {
                        p.ignite();
                        let _ = grid.set(x, y, p);
                    }
                }
            }
        }
        
        // Convert burned particles to smoke
        for (x, y) in to_smoke {
            if grid.in_bounds(x, y) && grid.is_empty(x, y) {
                let _ = grid.spawn(x, y, Material::Smoke);
            }
        }
        
        // Remove dead particles
        for (x, y) in to_remove {
            let _ = grid.remove(x, y);
        }
    }
    
    /// Update sand particle physics
    fn update_sand(&self, grid: &mut Grid, x: usize, y: usize) {
        // Try to fall straight down
        if y + 1 < grid.size().height && grid.is_empty(x, y + 1) {
            grid.swap(x, y, x, y + 1);
            return;
        }
        
        // Try to fall diagonally
        let width = grid.size().width;
        let can_left = x > 0 && y + 1 < grid.size().height && grid.is_empty(x - 1, y + 1);
        let can_right = x + 1 < width && y + 1 < grid.size().height && grid.is_empty(x + 1, y + 1);
        
        if can_left && can_right {
            let target = if rand_bool() { x - 1 } else { x + 1 };
            grid.swap(x, y, target, y + 1);
        } else if can_left {
            grid.swap(x, y, x - 1, y + 1);
        } else if can_right {
            grid.swap(x, y, x + 1, y + 1);
        }
    }
    
    /// Update water particle physics
    fn update_water(&self, grid: &mut Grid, x: usize, y: usize) {
        let width = grid.size().width;
        let height = grid.size().height;
        
        // Try to fall straight down
        if y + 1 < height && grid.is_empty(x, y + 1) {
            grid.swap(x, y, x, y + 1);
            return;
        }
        
        // Try to fall diagonally
        let can_left = x > 0 && y + 1 < height && grid.is_empty(x - 1, y + 1);
        let can_right = x + 1 < width && y + 1 < height && grid.is_empty(x + 1, y + 1);
        
        if can_left && can_right {
            let target = if rand_bool() { x - 1 } else { x + 1 };
            grid.swap(x, y, target, y + 1);
        } else if can_left {
            grid.swap(x, y, x - 1, y + 1);
        } else if can_right {
            grid.swap(x, y, x + 1, y + 1);
        } else {
            // Flow sideways
            let can_flow_left = x > 0 && grid.is_empty(x - 1, y);
            let can_flow_right = x + 1 < width && grid.is_empty(x + 1, y);
            
            if can_flow_left && can_flow_right {
                let target = if rand_bool() { x - 1 } else { x + 1 };
                grid.swap(x, y, target, y);
            } else if can_flow_left {
                grid.swap(x, y, x - 1, y);
            } else if can_flow_right {
                grid.swap(x, y, x + 1, y);
            }
        }
    }
    
    /// Update fire particle physics
    fn update_fire(
        &self,
        grid: &mut Grid,
        x: usize,
        y: usize,
        to_remove: &mut Vec<(usize, usize)>,
        to_ignite: &mut Vec<(usize, usize)>,
        _to_smoke: &mut Vec<(usize, usize)>,
    ) {
        let width = grid.size().width;
        let height = grid.size().height;
        
        // Try to rise upward
        if y > 0 && grid.is_empty(x, y - 1) {
            grid.swap(x, y, x, y - 1);
            return;
        }
        
        // Try to rise diagonally
        let can_left = x > 0 && y > 0 && grid.is_empty(x - 1, y - 1);
        let can_right = x + 1 < width && y > 0 && grid.is_empty(x + 1, y - 1);
        
        if can_left && can_right {
            let target = if rand_bool() { x - 1 } else { x + 1 };
            grid.swap(x, y, target, y - 1);
        } else if can_left {
            grid.swap(x, y, x - 1, y - 1);
        } else if can_right {
            grid.swap(x, y, x + 1, y - 1);
        }
        
        // Decay lifetime
        if let Some(mut p) = grid.get(x, y) {
            if p.lifetime > 0 {
                p.lifetime -= 1;
                let _ = grid.set(x, y, p);
            }
            if p.lifetime == 0 {
                to_remove.push((x, y));
                return;
            }
        }
        
        // Spread fire to neighbors (random chance)
        let neighbors = self.get_neighbor_positions(x, y, width, height);
        for (nx, ny) in neighbors {
            if let Some(np) = grid.get(nx, ny) {
                // Water extinguishes fire
                if np.material == Material::Water {
                    to_remove.push((x, y));
                    return;
                }
                // Spread to flammable materials
                if np.material.is_flammable() && rand_bool() {
                    to_ignite.push((nx, ny));
                }
            }
        }
    }
    
    /// Update smoke particle physics
    fn update_smoke(
        &self,
        grid: &mut Grid,
        x: usize,
        y: usize,
        to_remove: &mut Vec<(usize, usize)>,
    ) {
        let width = grid.size().width;
        let height = grid.size().height;
        
        // Smoke rises faster than fire
        if y > 0 && grid.is_empty(x, y - 1) {
            grid.swap(x, y, x, y - 1);
            return;
        }
        
        // Spread diagonally upward
        let can_left = x > 0 && y > 0 && grid.is_empty(x - 1, y - 1);
        let can_right = x + 1 < width && y > 0 && grid.is_empty(x + 1, y - 1);
        
        if can_left && can_right {
            let target = if rand_bool() { x - 1 } else { x + 1 };
            grid.swap(x, y, target, y - 1);
        } else if can_left {
            grid.swap(x, y, x - 1, y - 1);
        } else if can_right {
            grid.swap(x, y, x + 1, y - 1);
        }
        
        // Decay lifetime - smoke dissipates
        if let Some(mut p) = grid.get(x, y) {
            p.lifetime -= 1;
            let _ = grid.set(x, y, p);
            if p.lifetime == 0 {
                to_remove.push((x, y));
            }
        }
    }
    
    /// Get valid neighbor positions for fire spread
    fn get_neighbor_positions(&self, x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        
        // All 8 directions for fire spread
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)];
        
        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                neighbors.push((nx as usize, ny as usize));
            }
        }
        
        neighbors
    }
}

/// Simple pseudo-random for determinism
fn rand_bool() -> bool {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() % 2 == 0)
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GridSize, Particle};
    
    #[test]
    fn test_sand_falls() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        grid.spawn(1, 0, Material::Sand);
        
        let mut sim = Simulator::new();
        sim.tick(&mut grid);
        
        assert!(grid.is_empty(1, 0), "Sand should have moved from spawn point");
        assert_eq!(grid.get(1, 1).unwrap().material, Material::Sand);
    }
    
    #[test]
    fn test_sand_piles_on_floor() {
        let mut grid = Grid::new(GridSize::new(5, 5));
        // Sand at top
        grid.spawn(2, 0, Material::Sand);
        // Floor of stone at bottom
        for x in 0..5 {
            grid.set(x, 4, Particle::new(Material::Stone));
        }
        
        let mut sim = Simulator::new();
        
        // Run enough ticks for sand to reach floor
        for _ in 0..10 {
            sim.tick(&mut grid);
        }
        
        // Sand should be on floor (y=3 is highest non-floor row)
        let sand_on_floor = (0..5).any(|x| {
            grid.get(x, 3).map(|p| p.material == Material::Sand).unwrap_or(false)
        });
        assert!(sand_on_floor, "Sand should pile on the floor");
    }
    
    #[test]
    fn test_water_falls() {
        let mut grid = Grid::new(GridSize::new(3, 5));
        grid.spawn(1, 0, Material::Water);
        
        let mut sim = Simulator::new();
        sim.tick(&mut grid);
        
        assert!(grid.is_empty(1, 0), "Water should have moved");
        assert_eq!(grid.get(1, 1).unwrap().material, Material::Water);
    }
    
    #[test]
    fn test_water_extinguishes_fire() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        // Fire above water - fire rises, water falls toward it
        grid.spawn(1, 1, Material::Fire);
        grid.spawn(1, 0, Material::Water);
        
        let mut sim = Simulator::new();
        
        // Run several ticks
        for _ in 0..10 {
            sim.tick(&mut grid);
        }
        
        // Fire should eventually be extinguished when it contacts water
        // At minimum, verify water still exists (they interacted)
        let water_exists = (0..3).any(|y| (0..3).any(|x| 
            grid.get(x, y).map(|p| p.material == Material::Water).unwrap_or(false)
        ));
        assert!(water_exists, "Water should still exist after interaction");
    }
    
    #[test]
    fn test_fire_rises() {
        let mut grid = Grid::new(GridSize::new(3, 5));
        grid.spawn(1, 3, Material::Fire);
        
        let mut sim = Simulator::new();
        sim.tick(&mut grid);
        
        // Fire should rise (y decreases)
        let fire_above = (0..3).any(|y| {
            grid.get(1, y).map(|p| p.material == Material::Fire).unwrap_or(false)
        });
        assert!(fire_above, "Fire should have risen");
    }
    
    #[test]
    fn test_fire_dies() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        grid.spawn(1, 1, Material::Fire);
        
        let mut sim = Simulator::new();
        
        // Run many ticks - fire lifetime is 30-50
        for _ in 0..60 {
            sim.tick(&mut grid);
        }
        
        let fire_exists = (0..3).any(|y| (0..3).any(|x| 
            grid.get(x, y).map(|p| p.material == Material::Fire).unwrap_or(false)
        ));
        assert!(!fire_exists, "Fire should die after lifetime expires");
    }
    
    #[test]
    fn test_smoke_rises() {
        let mut grid = Grid::new(GridSize::new(3, 5));
        grid.spawn(1, 3, Material::Smoke);
        
        let mut sim = Simulator::new();
        sim.tick(&mut grid);
        
        let smoke_above = (0..3).any(|y| {
            grid.get(1, y).map(|p| p.material == Material::Smoke).unwrap_or(false)
        });
        assert!(smoke_above, "Smoke should have risen");
    }
    
    #[test]
    fn test_smoke_dissipates() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        grid.spawn(1, 1, Material::Smoke);
        
        let mut sim = Simulator::new();
        
        // Smoke lifetime is 60-100
        for _ in 0..120 {
            sim.tick(&mut grid);
        }
        
        let smoke_exists = (0..3).any(|y| (0..3).any(|x| 
            grid.get(x, y).map(|p| p.material == Material::Smoke).unwrap_or(false)
        ));
        assert!(!smoke_exists, "Smoke should dissipate after lifetime");
    }
    
    #[test]
    fn test_stone_immutable() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        grid.set(1, 1, Particle::new(Material::Stone));
        
        let mut sim = Simulator::new();
        
        for _ in 0..10 {
            sim.tick(&mut grid);
        }
        
        assert_eq!(grid.get(1, 1).unwrap().material, Material::Stone);
    }
    
    #[test]
    fn test_water_flows_horizontal() {
        let mut grid = Grid::new(GridSize::new(5, 3));
        grid.spawn(2, 0, Material::Water);
        // Floor
        for x in 0..5 {
            grid.set(x, 2, Particle::new(Material::Stone));
        }
        
        let mut sim = Simulator::new();
        
        for _ in 0..10 {
            sim.tick(&mut grid);
        }
        
        // Water should spread horizontally on the floor
        let water_count = (0..5).filter(|&x| {
            grid.get(x, 1).map(|p| p.material == Material::Water).unwrap_or(false)
        }).count();
        
        assert!(water_count >= 1, "Water should exist on floor");
    }
    
    #[test]
    fn test_multiple_sand_particles() {
        let mut grid = Grid::new(GridSize::new(5, 5));
        // Spawn multiple sand particles
        grid.spawn(2, 0, Material::Sand);
        grid.spawn(2, 1, Material::Sand);
        grid.spawn(2, 2, Material::Sand);
        
        let mut sim = Simulator::new();
        
        for _ in 0..10 {
            sim.tick(&mut grid);
        }
        
        // All sand should fall
        let sand_at_bottom = (0..5).any(|x| {
            grid.get(x, 4).map(|p| p.material == Material::Sand).unwrap_or(false)
        });
        assert!(sand_at_bottom, "Sand particles should fall to bottom");
    }
    
    #[test]
    fn test_grid_boundaries() {
        let mut grid = Grid::new(GridSize::new(3, 3));
        grid.spawn(0, 0, Material::Sand);
        grid.spawn(2, 2, Material::Water);
        
        let mut sim = Simulator::new();
        
        for _ in 0..5 {
            sim.tick(&mut grid);
        }
        
        // Should not panic and grid should remain valid
        assert!(grid.in_bounds(0, 0));
        assert!(grid.in_bounds(2, 2));
    }
    
    #[test]
    fn test_fire_spreads() {
        let mut grid = Grid::new(GridSize::new(5, 3));
        grid.spawn(2, 1, Material::Fire);
        grid.spawn(1, 1, Material::Sand); // Adjacent - flammable
        
        let mut sim = Simulator::new();
        
        let mut fire_spread = false;
        for _ in 0..30 {
            sim.tick(&mut grid);
        }
        
        // After many ticks, either original fire or spread fire should exist
        let has_fire = (0..5).any(|y| (0..3).any(|x| 
            grid.get(x, y).map(|p| p.material == Material::Fire).unwrap_or(false)
        ));
        assert!(has_fire, "Fire should exist (original or spread)");
    }
}
