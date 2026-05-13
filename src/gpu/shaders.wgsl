// Particle Simulation Compute Shader
// Falling Sand GPU Compute Pipeline - WGSL

struct Uniforms {
    width: u32,
    height: u32,
    tick: u32,
    _padding: u32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> particle_buffer: array<u32>;
@group(0) @binding(2) var<storage, read_write> particle_buffer_out: array<u32>;

// Material enum values (must match Rust side)
const MATERIAL_AIR: u32 = 0u;
const MATERIAL_SAND: u32 = 1u;
const MATERIAL_WATER: u32 = 2u;
const MATERIAL_STONE: u32 = 3u;
const MATERIAL_FIRE: u32 = 4u;
const MATERIAL_SMOKE: u32 = 5u;
const MATERIAL_BLACK_HOLE: u32 = 6u;
const MATERIAL_STEAM: u32 = 7u;
const MATERIAL_ICE: u32 = 8u;
const MATERIAL_OIL: u32 = 9u;
const MATERIAL_WOOD: u32 = 10u;
const MATERIAL_LAVA: u32 = 11u;
const MATERIAL_ASH: u32 = 12u;

// Helper to check if material has gravity
fn has_gravity(mat: u32) -> bool {
    return mat == MATERIAL_SAND ||
           mat == MATERIAL_WATER ||
           mat == MATERIAL_OIL ||
           mat == MATERIAL_ICE ||
           mat == MATERIAL_ASH;
}

// Helper to check if material rises
fn rises(mat: u32) -> bool {
    return mat == MATERIAL_FIRE ||
           mat == MATERIAL_SMOKE ||
           mat == MATERIAL_STEAM;
}

// Helper to check if material can move
fn can_move(mat: u32) -> bool {
    return mat != MATERIAL_STONE &&
           mat != MATERIAL_WOOD &&
           mat != MATERIAL_BLACK_HOLE;
}

// Get grid index from position
fn get_index(x: u32, y: u32, width: u32) -> u32 {
    return y * width + x;
}

// Get position from index
fn get_position(index: u32, width: u32) -> vec2<u32> {
    return vec2<u32>(index % width, index / width);
}

// Check if position is in bounds
fn in_bounds(x: i32, y: i32, width: u32, height: u32) -> bool {
    return x >= 0 && x < i32(width) && y >= 0 && y < i32(height);
}

// Get material from encoded particle data
fn get_material(particle: u32) -> u32 {
    return particle & 0x1Fu; // First 5 bits for material
}

// Set particle in output buffer (atomic-free, single pass)
fn set_particle(index: u32, material: u32) {
    particle_buffer_out[index] = material;
}

// Main compute shader - processes all particles
// Each workgroup processes 256 particles in parallel
@compute
@workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let width = uniforms.width;
    let height = uniforms.height;
    let index = global_id.x;
    
    // Check bounds
    if (index >= width * height) {
        return;
    }
    
    let pos = get_position(index, width);
    let x = pos.x;
    let y = pos.y;
    
    // Copy current particle to output
    var current = particle_buffer[index];
    var material = get_material(current);
    
    // Skip immovable materials (copy as-is)
    if (!can_move(material)) {
        set_particle(index, current);
        return;
    }
    
    // Check below for falling materials
    if (has_gravity(material)) {
        if (y + 1u < height) {
            let below_idx = get_index(x, y + 1u, width);
            let below = particle_buffer[below_idx];
            let below_mat = get_material(below);
            
            // Empty below - fall straight down
            if (below_mat == MATERIAL_AIR) {
                set_particle(index, MATERIAL_AIR);
                set_particle(below_idx, current);
                return;
            }
            
            // Try diagonal fall
            let go_left = (index + uniforms.tick) % 2u == 0u;
            
            if (go_left && x > 0u) {
                let diag_idx = get_index(x - 1u, y + 1u, width);
                if (get_material(particle_buffer[diag_idx]) == MATERIAL_AIR) {
                    set_particle(index, MATERIAL_AIR);
                    set_particle(diag_idx, current);
                    return;
                }
            } else if (x + 1u < width) {
                let diag_idx = get_index(x + 1u, y + 1u, width);
                if (get_material(particle_buffer[diag_idx]) == MATERIAL_AIR) {
                    set_particle(index, MATERIAL_AIR);
                    set_particle(diag_idx, current);
                    return;
                }
            }
        }
        
        // Water special: horizontal flow if can't fall
        if (material == MATERIAL_WATER) {
            let flow_left = (index + uniforms.tick) % 2u == 0u;
            
            if (flow_left && x > 0u) {
                let left_idx = get_index(x - 1u, y, width);
                if (get_material(particle_buffer[left_idx]) == MATERIAL_AIR) {
                    set_particle(index, MATERIAL_AIR);
                    set_particle(left_idx, current);
                    return;
                }
            } else if (x + 1u < width) {
                let right_idx = get_index(x + 1u, y, width);
                if (get_material(particle_buffer[right_idx]) == MATERIAL_AIR) {
                    set_particle(index, MATERIAL_AIR);
                    set_particle(right_idx, current);
                    return;
                }
            }
        }
    }
    
    // Materials that rise (fire, smoke, steam)
    if (rises(material)) {
        if (y > 0u) {
            let above_idx = get_index(x, y - 1u, width);
            let above = particle_buffer[above_idx];
            let above_mat = get_material(above);
            
            // Empty above - rise
            if (above_mat == MATERIAL_AIR) {
                set_particle(index, MATERIAL_AIR);
                set_particle(above_idx, current);
                return;
            }
            
            // Try diagonal rise
            let go_left = (index + uniforms.tick) % 2u == 0u;
            
            if (go_left && x > 0u) {
                let diag_idx = get_index(x - 1u, y - 1u, width);
                if (get_material(particle_buffer[diag_idx]) == MATERIAL_AIR) {
                    set_particle(index, MATERIAL_AIR);
                    set_particle(diag_idx, current);
                    return;
                }
            } else if (x + 1u < width) {
                let diag_idx = get_index(x + 1u, y - 1u, width);
                if (get_material(particle_buffer[diag_idx]) == MATERIAL_AIR) {
                    set_particle(index, MATERIAL_AIR);
                    set_particle(diag_idx, current);
                    return;
                }
            }
        }
    }
    
    // Default: copy particle as-is
    set_particle(index, current);
}