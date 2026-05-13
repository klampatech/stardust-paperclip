#!/usr/bin/env python3
"""
FUL-8c Verification Script
Run this to verify all FUL-8c deliverables are complete.

Usage: python3 verify_ful_8c.py
"""

import os
import sys

def check_file_exists(filepath, description):
    """Check if a file exists and return status."""
    if os.path.exists(filepath):
        print(f"✓ {description}: {filepath}")
        return True
    else:
        print(f"✗ {description}: {filepath} (MISSING)")
        return False

def check_file_contains(filepath, search_string, description):
    """Check if a file contains a specific string."""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
            if search_string in content:
                print(f"✓ {description}")
                return True
            else:
                print(f"✗ {description}")
                return False
    except Exception as e:
        print(f"✗ {description} (Error: {e})")
        return False

def main():
    print("=" * 60)
    print("FUL-8c: Integration & Performance Benchmarking")
    print("Verification Script")
    print("=" * 60)
    print()
    
    all_pass = True
    
    # 1. Check Spatial Hash Integration
    print("[1] Spatial Hash Integration")
    print("-" * 40)
    all_pass &= check_file_contains(
        "src/simulation.rs",
        "GridSpatialHash::new",
        "  tick_chunked() uses GridSpatialHash"
    )
    all_pass &= check_file_contains(
        "src/simulation.rs",
        "SpatialHashTrait",
        "  Generic trait for spatial hash"
    )
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "pub trait SpatialHashTrait",
        "  SpatialHashTrait defined in chunk.rs"
    )
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "impl SpatialHashTrait for GridSpatialHash",
        "  GridSpatialHash implements SpatialHashTrait"
    )
    print()
    
    # 2. Check Dirty Chunk Tracking
    print("[2] Dirty Chunk Tracking")
    print("-" * 40)
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "dirty_chunks",
        "  dirty_chunks HashSet in ChunkedGrid"
    )
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "mark_dirty",
        "  mark_dirty() method exists"
    )
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "wake_neighbor_chunks",
        "  wake_neighbor_chunks() method exists"
    )
    print()
    
    # 3. Check Sleeping Particle System
    print("[3] Sleeping Particle System")
    print("-" * 40)
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "sleeping_particles",
        "  sleeping_particles HashSet exists"
    )
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "mark_sleeping",
        "  mark_sleeping() method exists"
    )
    all_pass &= check_file_contains(
        "src/chunk.rs",
        "is_sleeping",
        "  is_sleeping() method exists"
    )
    print()
    
    # 4. Check GPU Pipeline
    print("[4] GPU Pipeline")
    print("-" * 40)
    all_pass &= check_file_exists(
        "src/gpu/compute.rs",
        "GPU compute pipeline"
    )
    all_pass &= check_file_exists(
        "src/gpu/shaders.wgsl",
        "WGSL compute shader"
    )
    all_pass &= check_file_contains(
        "src/gpu/compute.rs",
        "GpuSimulator",
        "GpuSimulator class exists"
    )
    print()
    
    # 5. Check Benchmark Harness
    print("[5] Benchmark Harness")
    print("-" * 40)
    all_pass &= check_file_exists(
        "src/benchmark.rs",
        "Benchmark harness"
    )
    all_pass &= check_file_contains(
        "src/benchmark.rs",
        "SpatialBenchmark",
        "SpatialBenchmark struct exists"
    )
    print()
    
    # 6. Check Integration Tests
    print("[6] Integration Tests")
    print("-" * 40)
    all_pass &= check_file_exists(
        "examples/ful_8c_integration_test.rs",
        "Full integration test"
    )
    all_pass &= check_file_exists(
        "examples/grid_spatial_benchmark.rs",
        "GridSpatialHash benchmark"
    )
    all_pass &= check_file_exists(
        "examples/gpu_integration_demo.rs",
        "GPU integration demo"
    )
    all_pass &= check_file_exists(
        "examples/spatial_integration_test.rs",
        "Spatial hash integration test"
    )
    print()
    
    # 7. Check Documentation
    print("[7] Documentation")
    print("-" * 40)
    all_pass &= check_file_exists(
        "FUL-8c_COMPLETE.md",
        "Complete documentation"
    )
    all_pass &= check_file_exists(
        "FUL-8c_HANDOFF.md",
        "Handoff document"
    )
    all_pass &= check_file_exists(
        "FUL-8c_STATUS.md",
        "Status report"
    )
    all_pass &= check_file_exists(
        "FUL-8c_PROFILING_REPORT.md",
        "Profiling report"
    )
    all_pass &= check_file_exists(
        "FUL-8c_OPTIMIZATION_PLAN.md",
        "Optimization plan"
    )
    print()
    
    # 8. Check Profiling Report Content
    print("[8] Profiling Report Content")
    print("-" * 40)
    all_pass &= check_file_contains(
        "FUL-8c_PROFILING_REPORT.md",
        "500K",
        "  500K target documented"
    )
    all_pass &= check_file_contains(
        "FUL-8c_PROFILING_REPORT.md",
        "bottleneck",
        "  Bottleneck analysis present"
    )
    all_pass &= check_file_contains(
        "FUL-8c_PROFILING_REPORT.md",
        "GPU",
        "  GPU acceleration path documented"
    )
    print()
    
    # Summary
    print("=" * 60)
    if all_pass:
        print("✓ ALL FUL-8c DELIVERABLES VERIFIED")
    else:
        print("✗ SOME DELIVERABLES MISSING")
    print("=" * 60)
    print()
    
    print("Run verification commands:")
    print("  cargo run --example ful_8c_integration_test")
    print("  cargo run --example grid_spatial_benchmark")
    print("  cargo run --example gpu_integration_demo")
    print()
    
    return 0 if all_pass else 1

if __name__ == "__main__":
    sys.exit(main())