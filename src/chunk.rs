//! Spatial partitioning for efficient particle operations
//! 
//! Uses a chunk-based system (64x64 cells per chunk) to optimize
//! neighbor lookups and enable efficient simulation of 50,000+ particles.

use crate::grid::{Grid, GridSize};
use crate::particle::{Material, Particle};

/// Chunk size (power of 2 for efficient math)
pub const CHUNK_SIZE: usize = 64;

/// Number of cells in a chunk
pub const CHUNK_CELLS: usize = CHUNK_SIZE * CHUNK_SIZE;

/// Chunk coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    /// Convert world position to chunk position
    pub fn from_world(x: usize, y: usize) -> Self {
        Self {
            x: (x / CHUNK_SIZE) as i32,
            y: (y / CHUNK_SIZE) as i32,
        }
    }
}

/// A single chunk containing particles
#[derive(Debug, Clone)]
pub struct Chunk {
    pub pos: ChunkPos,
    /// Flat array of particles (CHUNK_SIZE x CHUNK_SIZE)
    pub cells: Vec<Particle>,
    /// Particle count for quick checks
    particle_count: usize,
}

impl Chunk {
    /// Create a new empty chunk
    pub fn new(pos: ChunkPos) -> Self {
        Self {
            pos,
            cells: vec![Particle::empty(); CHUNK_CELLS],
            particle_count: 0,
        }
    }
    
    /// Get particle at local chunk coordinates
    pub fn get_local(&self, lx: usize, ly: usize) -> Option<Particle> {
        if lx < CHUNK_SIZE && ly < CHUNK_SIZE {
            Some(self.cells[ly * CHUNK_SIZE + lx])
        } else {
            None
        }
    }
    
    /// Set particle at local chunk coordinates
    pub fn set_local(&mut self, lx: usize, ly: usize, particle: Particle) -> bool {
        if lx < CHUNK_SIZE && ly < CHUNK_SIZE {
            let idx = ly * CHUNK_SIZE + lx;
            let was_empty = self.cells[idx].material == Material::Air;
            let now_empty = particle.material == Material::Air;
            
            self.cells[idx] = particle;
            
            // Update particle count
            if was_empty && !now_empty {
                self.particle_count += 1;
            } else if !was_empty && now_empty {
                self.particle_count = self.particle_count.saturating_sub(1);
            }
            true
        } else {
            false
        }
    }
    
    /// Check if local position is empty
    pub fn is_empty_local(&self, lx: usize, ly: usize) -> bool {
        self.get_local(lx, ly)
            .map(|p| p.material == Material::Air)
            .unwrap_or(false)
    }
    
    /// Get particle count
    pub fn particle_count(&self) -> usize {
        self.particle_count
    }
    
    /// Check if chunk is empty
    pub fn is_empty(&self) -> bool {
        self.particle_count == 0
    }
    
    /// Check if chunk is full
    pub fn is_full(&self) -> bool {
        self.particle_count >= CHUNK_CELLS
    }
}

/// World position within a chunk
#[derive(Debug, Clone, Copy)]
pub struct ChunkLocalPos {
    pub chunk: ChunkPos,
    pub lx: usize,
    pub ly: usize,
}

impl ChunkLocalPos {
    /// Convert world position to chunk-local position
    pub fn from_world(x: usize, y: usize) -> Self {
        Self {
            chunk: ChunkPos::from_world(x, y),
            lx: x % CHUNK_SIZE,
            ly: y % CHUNK_SIZE,
        }
    }
}

/// Chunked grid using spatial partitioning
#[derive(Debug, Clone)]
pub struct ChunkedGrid {
    size: GridSize,
    chunks: std::collections::HashMap<ChunkPos, Chunk>,
    chunk_count_x: i32,
    chunk_count_y: i32,
    /// Set of chunks modified since last tick (for dirty tracking)
    dirty_chunks: std::collections::HashSet<ChunkPos>,
}

impl ChunkedGrid {
    /// Create a new chunked grid
    pub fn new(size: GridSize) -> Self {
        let chunk_count_x = ((size.width + CHUNK_SIZE - 1) / CHUNK_SIZE) as i32;
        let chunk_count_y = ((size.height + CHUNK_SIZE - 1) / CHUNK_SIZE) as i32;
        
        Self {
            size,
            chunks: std::collections::HashMap::new(),
            chunk_count_x,
            chunk_count_y,
            dirty_chunks: std::collections::HashSet::new(),
        }
    }
    
    /// Get grid dimensions
    pub fn size(&self) -> GridSize {
        self.size
    }
    
    /// Get or create a chunk at position
    fn get_chunk_mut(&mut self, pos: ChunkPos) -> &mut Chunk {
        if !self.chunks.contains_key(&pos) {
            self.chunks.insert(pos, Chunk::new(pos));
        }
        self.chunks.get_mut(&pos).unwrap()
    }
    
    /// Get chunk at position (immutable)
    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }
    
    /// Get particle at world position
    pub fn get(&self, x: usize, y: usize) -> Option<Particle> {
        if !self.size.contains(x, y) {
            return None;
        }
        
        let local = ChunkLocalPos::from_world(x, y);
        self.get_chunk(local.chunk)?.get_local(local.lx, local.ly)
    }
    
    /// Set particle at world position
    pub fn set(&mut self, x: usize, y: usize, particle: Particle) -> bool {
        if !self.size.contains(x, y) {
            return false;
        }
        
        let local = ChunkLocalPos::from_world(x, y);
        let chunk = self.get_chunk_mut(local.chunk);
        let changed = chunk.set_local(local.lx, local.ly, particle);
        
        // Mark chunk as dirty if content changed
        if changed {
            self.dirty_chunks.insert(local.chunk);
        }
        
        changed
    }
    
    /// Check if cell is empty
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get(x, y)
            .map(|p| p.material == Material::Air)
            .unwrap_or(false)
    }
    
    /// Check if position is in bounds
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        self.size.contains(x, y)
    }
    
    /// Spawn particle if cell is empty
    pub fn spawn(&mut self, x: usize, y: usize, material: Material) -> bool {
        if !self.size.contains(x, y) {
            return false;
        }
        
        let local = ChunkLocalPos::from_world(x, y);
        let chunk = self.get_chunk_mut(local.chunk);
        
        // Only spawn if empty
        if chunk.is_empty_local(local.lx, local.ly) {
            chunk.set_local(local.lx, local.ly, Particle::new(material));
            self.dirty_chunks.insert(local.chunk);
            true
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
            let local1 = ChunkLocalPos::from_world(x1, y1);
            let local2 = ChunkLocalPos::from_world(x2, y2);
            
            // Handle same-chunk and cross-chunk swaps differently
            if local1.chunk == local2.chunk {
                // Same chunk - we can do it safely
                if let Some(chunk) = self.chunks.get_mut(&local1.chunk) {
                    chunk.set_local(local1.lx, local1.ly, p2);
                    chunk.set_local(local2.lx, local2.ly, p1);
                }
            } else {
                // Cross-chunk swap - need to get both chunks but handle carefully
                // First get p1's chunk and set p2 there
                let chunk1 = self.get_chunk_mut(local1.chunk);
                chunk1.set_local(local1.lx, local1.ly, p2);
                
                // Then get p2's chunk and set p1 there
                let chunk2 = self.get_chunk_mut(local2.chunk);
                chunk2.set_local(local2.lx, local2.ly, p1);
            }
            
            // Mark affected chunks as dirty
            self.dirty_chunks.insert(local1.chunk);
            self.dirty_chunks.insert(local2.chunk);
        }
    }
    
    /// Get total particle count across all chunks
    pub fn total_particles(&self) -> usize {
        self.chunks.values().map(|c| c.particle_count()).sum()
    }
    
    /// Get all dirty chunks that need processing
    pub fn dirty_chunks(&self) -> &std::collections::HashSet<ChunkPos> {
        &self.dirty_chunks
    }
    
    /// Get all dirty chunks as a mutable reference for processing
    pub fn dirty_chunks_mut(&mut self) -> &mut std::collections::HashSet<ChunkPos> {
        &mut self.dirty_chunks
    }
    
    /// Mark a chunk as dirty (e.g., when a neighbor chunk's particle affected it)
    pub fn mark_dirty(&mut self, pos: ChunkPos) {
        self.dirty_chunks.insert(pos);
    }
    
    /// Clear dirty flags after processing
    pub fn clear_dirty(&mut self) {
        self.dirty_chunks.clear();
    }
    
    /// Get all chunks that need processing
    pub fn active_chunks(&self) -> Vec<&Chunk> {
        self.chunks.values().filter(|c| !c.is_empty()).collect()
    }
    
    /// Convert back to flat grid (for testing/compatibility)
    pub fn to_flat_grid(&self) -> Grid {
        let mut grid = Grid::new(self.size);
        
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                if let Some(p) = self.get(x, y) {
                    let _ = grid.set(x, y, p);
                }
            }
        }
        
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new(ChunkPos::new(0, 0));
        assert_eq!(chunk.particle_count(), 0);
        assert!(chunk.is_empty());
    }
    
    #[test]
    fn test_chunk_particle_operations() {
        let mut chunk = Chunk::new(ChunkPos::new(0, 0));
        
        // Set a particle
        assert!(chunk.set_local(10, 10, Particle::new(Material::Sand)));
        assert_eq!(chunk.particle_count(), 1);
        assert!(!chunk.is_empty_local(10, 10));
        
        // Remove particle
        chunk.set_local(10, 10, Particle::empty());
        assert_eq!(chunk.particle_count(), 0);
        assert!(chunk.is_empty_local(10, 10));
    }
    
    #[test]
    fn test_chunked_grid() {
        let mut grid = ChunkedGrid::new(GridSize::new(100, 100));
        
        // Spawn particles
        assert!(grid.spawn(50, 50, Material::Sand));
        assert!(grid.spawn(10, 10, Material::Water));
        
        assert_eq!(grid.total_particles(), 2);
        
        // Check particles
        assert_eq!(grid.get(50, 50).unwrap().material, Material::Sand);
        assert_eq!(grid.get(10, 10).unwrap().material, Material::Water);
    }
    
    #[test]
    fn test_chunk_boundaries() {
        let mut grid = ChunkedGrid::new(GridSize::new(128, 128));
        
        // Spawn at chunk boundary (64, 64)
        assert!(grid.spawn(64, 64, Material::Fire));
        
        let local = ChunkLocalPos::from_world(64, 64);
        assert_eq!(local.chunk.x, 1);
        assert_eq!(local.chunk.y, 1);
        assert_eq!(local.lx, 0);
        assert_eq!(local.ly, 0);
    }
}
