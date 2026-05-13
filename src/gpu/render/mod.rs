//! Rendering Pipeline for GPU-accelerated particle visualization
//! 
//! Provides instanced rendering with compute shader culling and LOD system.

#[cfg(feature = "gpu")]
pub mod render;

#[cfg(feature = "gpu")]
pub use render::*;