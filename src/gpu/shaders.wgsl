// Particle Simulation Compute Shader
// Falls Sand GPU Compute Pipeline

// Simulation uniforms
struct Uniforms {
    width: u32,
    height: u32,
    tick: u32,
    _padding: u32,
}

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
// Encoding: bits 0-4 = material (13 values), bits 5-31 = state flags
fn get_material(particle: u32) -> u32 {
    return particle & 0x1Fu; // First 5 bits
}

// Encode particle data
fn encode_particle(material: u32, flags: u32) -> u32 {
    return material | (flags << 5);
}

// Main compute shader
@compute
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let width = uniform.width;
    let height = uniform.height;
    let index = global_id.x;
    
    // Check bounds
    if (index >= width * height) {
        return;
    }
    
    let pos = get_position(index, width);
    let x = pos.x;
    let y = pos.y;
    
    // Get current particle
    let current = particle_buffer[index];
    let material = get_material(current);
    
    // Skip immovable materials
    if (!can_move(material)) {
        return;
    }
    
    // Process based on material type
    var new_x = i32(x);
    var new_y = i32(y);
    var should_swap = false;
    
    // Materials with gravity (fall down)
    if (has_gravity(material)) {
        // Check below
        if (in_bounds(i32(x), i32(y) + 1, width, height)) {
            let below_idx = get_index(u32(x), u32(y) + 1, width);
            let below = particle_buffer[below_idx];
            let below_mat = get_material(below);
            
            // Empty below - fall straight down
            if (below_mat == MATERIAL_AIR) {
                new_y = i32(y) + 1;
                should_swap = true;
            }
            // Fall diagonally
            else if (!should_swap) {
                // Randomize direction for even spread
                let go_left = (index + uniform.tick) % 2u == 0u;
                
                if (go_left && x > 0u) {
                    let diag_idx = get_index(x - 1u, u32(new_y), width);
                    let diag = particle_buffer[diag_idx];
                    if (get_material(diag) == MATERIAL_AIR) {
                        new_x = i32(x) - 1;
                        should_swap = true;
                    }
                } else if (x + 1u < width) {
                    let diag_idx = get_index(x + 1u, u32(new_y), width);
                    let diag = particle_buffer[diag_idx];
                    if (get_material(diag) == MATERIAL_AIR) {
                        new_x = i32(x) + 1;
                        should_swap = true;
                    }
                }
            }
        }
    }
    // Materials that rise (fire, smoke, steam)
    else if (rises(material)) {
        // Check above
        if (in_bounds(i32(x), i32(y) - 1, width, height)) {
            let above_idx = get_index(u32(x), u32(y) - 1, width);
            let above = particle_buffer[above_idx];
            let above_mat = get_material(above);
            
            // Empty above - rise
            if (above_mat == MATERIAL_AIR) {
                new_y = i32(y) - 1;
                should_swap = true;
            }
            // Rise diagonally
            else if (!should_swap) {
                let go_left = (index + uniform.tick) % 2u == 0u;
                
                if (go_left && x > 0u) {
                    let diag_idx = get_index(x - 1u, u32(new_y), width);
                    let diag = particle_buffer[diag_idx];
                    if (get_material(diag) == MATERIAL_AIR) {
                        new_x = i32(x) - 1;
                        should_swap = true;
                    }
                } else if (x + 1u < width) {
                    let diag_idx = get_index(x + 1u, u32(new_y), width);
                    let diag = particle_buffer[diag_idx];
                    if (get_material(diag) == MATERIAL_AIR) {
                        new_x = i32(x) + 1;
                        should_swap = true;
                    }
                }
            }
        }
    }
    // Water special behavior - also flows horizontally
    else if (material == MATERIAL_WATER) {
        // Try to fall
        if (in_bounds(i32(x), i32(y) + 1, width, height)) {
            let below_idx = get_index(u32(x), u32(y) + 1, width);
            if (get_material(particle_buffer[below_idx]) == MATERIAL_AIR) {
                new_y = i32(y) + 1;
                should_swap = true;
            }
        }
        // If can't fall, try horizontal flow
        else if (!should_swap) {
            let flow_left = (index + uniform.tick) % 2u == 0u;
            
            if (flow_left && x > 0u) {
                let left_idx = get_index(x - 1u, u32(y), width);
                if (get_material(particle_buffer[left_idx]) == MATERIAL_AIR) {
                    new_x = i32(x) - 1;
                    should_swap = true;
                }
            } else if (x + 1u < width) {
                let right_idx = get_index(x + 1u, u32(y), width);
                if (get_material(particle_buffer[right_idx]) == MATERIAL_AIR) {
                    new_x = i32(x) + 1;
                    should_swap = true;
                }
            }
        }
    }
    
    // Perform swap if needed (atomic to avoid race conditions)
    // Note: In real implementation, would use atomic operations
    // This is simplified - actual GPU implementation needs careful synchronization
    if (should_swap) {
        let target_idx = get_index(u32(new_x), u32(new_y), width);
        
        // Swap in buffer
        // Would need proper synchronization for real GPU implementation
        let temp = particle_buffer[target_idx];
        // particle_buffer[target_idx] = current;
        // particle_buffer[index] = temp;
    }
}

// Render shader for visualization
@fragment
fn render_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    // This would be in a separate render pipeline
    // For now, just output white placeholder
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}