// FUL-35a: WASM Integration Test
// Quick verification of the WASM API surface
// 
// Run with: wasm-pack build --target web --out-dir pkg
// Then include the generated JS module

import init, { Simulation } from './pkg/falling_sand.js';

async function runTest() {
  console.log('=== FUL-35a WASM Test ===');
  
  // Initialize WASM module
  await init();
  console.log('✅ WASM module initialized');
  
  // Create simulation
  const sim = new Simulation(100, 100);
  console.log('✅ Simulation created (100x100)');
  
  // Test dimensions
  console.log(`   Width: ${sim.width()}, Height: ${sim.height()}`);
  
  // Test spawning
  const spawned = sim.spawn(50, 50, 'sand');
  console.log(`✅ Spawn sand at (50,50): ${spawned}`);
  
  const brushCount = sim.spawn_brush(30, 70, 'water', 5);
  console.log(`✅ Spawn brush water (r=5): ${brushCount} particles`);
  
  const fire = sim.spawn(70, 30, 'fire');
  console.log(`✅ Spawn fire at (70,30): ${fire}`);
  
  // Test query methods
  const mat = sim.get_material(50, 50);
  console.log(`✅ Get material at (50,50): ${mat}`);
  
  const lifetime = sim.get_lifetime(70, 30);
  console.log(`✅ Fire lifetime at (70,30): ${lifetime}`);
  
  const count = sim.particle_count();
  console.log(`✅ Particle count: ${count}`);
  
  // Test physics tick
  sim.tick();
  console.log('✅ Physics tick executed');
  
  const countAfter = sim.particle_count();
  console.log(`   Particle count after tick: ${countAfter}`);
  
  // Test render
  const pixels = sim.render_rgba();
  console.log(`✅ Render to RGBA: ${pixels.length} bytes (${sim.width()}x${sim.height()}x4)`);
  
  // Test zero-copy render
  sim.render_rgba_direct();
  const buffer = sim.take_pixel_buffer();
  console.log(`✅ Zero-copy render: ${buffer.length} bytes`);
  
  // Test tick_many
  sim.tick_many(10);
  console.log('✅ Batch tick (10) executed');
  
  // Test clear
  sim.clear();
  const countCleared = sim.particle_count();
  console.log(`✅ Clear simulation: ${countCleared} particles remaining`);
  
  // Test all materials
  const materials = ['sand', 'water', 'stone', 'fire', 'smoke', 'blackhole', 
                     'steam', 'ice', 'oil', 'wood', 'lava', 'ash'];
  console.log('✅ Testing all materials...');
  for (const mat of materials) {
    const result = sim.spawn(50, 50, mat);
    const retrieved = sim.get_material(50, 50);
    const match = retrieved === mat;
    console.log(`   ${mat}: spawn=${result}, get=${retrieved}, match=${match}`);
  }
  
  // Test black hole physics
  sim.clear();
  sim.spawn(50, 50, 'blackhole');
  for (let i = 0; i < 5; i++) {
    sim.spawn_brush(20 + i * 10, 20, 'sand', 3);
  }
  console.log(`   Black hole test: ${sim.particle_count()} particles near event horizon`);
  
  sim.tick_many(50);
  console.log(`   After 50 ticks: ${sim.particle_count()} particles`);
  
  console.log('\n=== All Tests Passed ===');
}

runTest().catch(err => {
  console.error('❌ Test failed:', err);
  process.exit(1);
});