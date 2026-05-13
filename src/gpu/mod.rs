//! GPU Compute Shader Pipeline
//! 
//! Implements parallel particle simulation using WebGPU compute shaders.
//! Provides significant performance improvements for 50,000+ particles.

#[cfg(feature = "gpu")]
mod compute;

#[cfg(feature = "gpu")]
pub use compute::*;