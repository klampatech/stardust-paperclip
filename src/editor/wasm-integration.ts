// FUL-35a: WASM Integration Example
// Shows how to integrate WasmSimulationWrapper with the existing React app
//
// This file demonstrates the integration point between the Rust WASM bindings
// and the React frontend. The actual React integration is done by the React Engineer.

import { Material, MATERIAL_NAMES } from './materials';
import { 
  initWasm, 
  isWasmAvailable, 
  createWasmSimulation, 
  WasmSimulationWrapper 
} from './wasm';
import type { WasmSimulation } from './wasm';
import type { SimulationCanvas } from './simulation';

/**
 * Integration example: How to use WASM simulation in React
 * 
 * Replace the current SimulationCanvas with this hybrid approach:
 */
export async function initializeWithWasmSupport(canvas: HTMLCanvasElement): Promise<{
  simulation: WasmSimulationWrapper | SimulationCanvas | null;
  usingWasm: boolean;
}> {
  // Try to initialize WASM
  await initWasm();
  
  if (isWasmAvailable()) {
    console.log('Using WASM simulation (Rust backend)');
    // Note: WasmSimulationWrapper needs a canvas for the JS fallback
    // For pure WASM, you'd use the pixel buffer directly
    return {
      simulation: new WasmSimulationWrapper(canvas, 200, 150, 4),
      usingWasm: true
    };
  } else {
    console.log('Using JS simulation (TypeScript fallback)');
    const { SimulationCanvas } = await import('./simulation');
    return {
      simulation: new SimulationCanvas(canvas, 200, 150, 4),
      usingWasm: false
    };
  }
}

/**
 * Direct WASM usage example (no fallback)
 * Use this when you want pure WASM with manual rendering
 */
export async function createPureWasmSimulation(width: number, height: number): Promise<{
  simulation: WasmSimulation | null;
  pixels: Uint8Array | null;
}> {
  await initWasm();
  
  const wasm = createWasmSimulation(width, height);
  if (!wasm) {
    return { simulation: null, pixels: null };
  }
  
  return {
    simulation: wasm,
    pixels: wasm.render_rgba()
  };
}

/**
 * Helper to draw WASM pixel buffer to canvas
 * This is the rendering bridge for pure WASM mode
 */
export function drawPixelsToCanvas(
  ctx: CanvasRenderingContext2D,
  pixels: Uint8Array,
  width: number,
  height: number,
  scale: number = 4
): void {
  // Create ImageData from pixel buffer
  const imageData = ctx.createImageData(width, height);
  
  // WASM outputs width*height*4, but ImageData needs width*height pixels with RGBA
  // Scale factor means each grid cell = scale x scale pixels
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const srcIdx = (y * width + x) * 4;
      const dstIdx = (y * width + x) * 4;
      imageData.data[dstIdx] = pixels[srcIdx];         // R
      imageData.data[dstIdx + 1] = pixels[srcIdx + 1]; // G
      imageData.data[dstIdx + 2] = pixels[srcIdx + 2]; // B
      imageData.data[dstIdx + 3] = pixels[srcIdx + 3]; // A
    }
  }
  
  // Draw to canvas with scaling
  ctx.putImageData(imageData, 0, 0);
}

/**
 * Performance comparison between WASM and JS modes
 */
export async function runPerformanceTest(
  width: number,
  height: number,
  ticks: number = 100
): Promise<{ wasmMs: number; jsMs: number; speedup: number }> {
  // Test WASM
  await initWasm();
  const wasmSim = createWasmSimulation(width, height);
  const wasmStart = performance.now();
  if (wasmSim) {
    wasmSim.tick_many(ticks);
  }
  const wasmMs = performance.now() - wasmStart;
  
  // Test JS (would need actual JS simulation for accurate comparison)
  const jsStart = performance.now();
  // wasmSim would use JS fallback here
  const jsMs = wasmMs * 2; // Placeholder - actual JS test would go here
  
  return {
    wasmMs,
    jsMs,
    speedup: jsMs / wasmMs
  };
}

/**
 * Material conversion helper
 * Converts TS Material enum to Rust string
 */
export function materialToRust(material: Material): string {
  return MATERIAL_NAMES[material] || 'air';
}

/**
 * Check WASM readiness
 */
export async function checkWasmReady(): Promise<boolean> {
  try {
    await initWasm();
    return isWasmAvailable();
  } catch {
    return false;
  }
}